mod app;
mod event;
mod ui;

pub use app::{App, AppState, InputMode};
pub use event::{Event, EventHandler};

use crate::config::Config;
use crate::socket::SocketClient;
use crate::supervisor::Supervisor;
use crate::types::LogMessage;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::sync::mpsc;

/// Lance la TUI avec supervision du processus
pub async fn run_tui(
    project: String,
    command: Vec<String>,
    config: Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Créer l'application
    let mut app = App::new(project.clone(), command.clone(), config.clone());

    // Créer le channel pour les logs
    let (tx_log, rx_log) = mpsc::channel::<LogMessage>(config.performance.buffer_size);

    // Démarrer le socket worker
    let socket_path = config.agent.socket_path.clone();
    let socket_client = SocketClient::new(Some(socket_path));
    let (tx_socket, rx_socket) = mpsc::channel::<LogMessage>(config.performance.buffer_size);
    let socket_task = tokio::spawn(async move {
        let _ = socket_client.start_worker(rx_socket).await;
    });

    // Créer le superviseur
    let mut supervisor = Supervisor::new(project, command, config.clone());

    // Démarrer le processus
    match supervisor.start(tx_log.clone()).await {
        Ok(pid) => {
            app.set_pid(Some(pid));
            app.set_state(AppState::Running);
        }
        Err(e) => {
            app.add_system_log(format!("Failed to start process: {}", e));
            app.set_state(AppState::WaitingCountdown(5));
        }
    }

    // Créer le handler d'événements
    let tick_rate = std::time::Duration::from_millis(config.performance.tui.tick_rate_ms);
    let mut event_handler = EventHandler::new(tick_rate);
    
    // Frame rate limiter
    let frame_duration = std::time::Duration::from_millis(config.performance.tui.frame_rate_ms);
    let mut last_frame = std::time::Instant::now();

    // Boucle principale
    let mut channels = Channels {
        rx_log,
        tx_log: tx_log.clone(),
        tx_socket,
    };
    
    let result = run_app_loop(
        &mut terminal,
        &mut app,
        &mut supervisor,
        &mut event_handler,
        &mut channels,
        frame_duration,
        &mut last_frame,
        &config,
    )
    .await;

    // Cleanup with timeout to prevent hanging on quit
    let _ = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        supervisor.stop()
    ).await;
    drop(tx_log);
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        socket_task
    ).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Structure pour regrouper les channels de communication
struct Channels {
    rx_log: mpsc::Receiver<LogMessage>,
    tx_log: mpsc::Sender<LogMessage>,
    tx_socket: mpsc::Sender<LogMessage>,
}

async fn run_app_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    app: &mut App,
    supervisor: &mut Supervisor,
    event_handler: &mut EventHandler,
    channels: &mut Channels,
    frame_duration: std::time::Duration,
    last_frame: &mut std::time::Instant,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        // Dessiner l'interface seulement si nécessaire et si assez de temps s'est écoulé
        let now = std::time::Instant::now();
        if app.needs_redraw && now.duration_since(*last_frame) >= frame_duration {
            terminal.draw(|f| ui::draw(f, app))?;
            app.needs_redraw = false;
            *last_frame = now;
        }

        // Gérer les événements avec select
        tokio::select! {
            // Événement clavier/souris
            Some(event) = event_handler.next() => {
                match event {
                    Event::Key(key) => {
                        use crossterm::event::KeyCode;
                        
                        // Handle 'q' globally to quit from any mode
                        if let KeyCode::Char('q') = key.code {
                            app.should_quit = true;
                            continue;
                        }
                        
                        // Gestion des inputs selon le mode
                        match app.input_mode {
                            InputMode::Normal => {
                                match key.code {
                                    KeyCode::Char('r') => {
                                        app.add_system_log("Restarting...".to_string());
                                        app.set_state(AppState::Restarting);
                                        
                                        // Forcer le redraw immédiatement pour montrer "Restarting..."
                                        terminal.draw(|f| ui::draw(f, app))?;
                                        *last_frame = std::time::Instant::now();
                                        
                                        match supervisor.restart(channels.tx_log.clone()).await {
                                            Ok(pid) => {
                                                app.set_pid(Some(pid));
                                                app.set_state(AppState::Running);
                                                app.reset_start_time();
                                                app.add_system_log(format!("Process restarted (PID: {})", pid));
                                            }
                                            Err(e) => {
                                                app.add_system_log(format!("Restart failed: {}", e));
                                                app.set_state(AppState::WaitingCountdown(5));
                                            }
                                        }
                                    }
                                    KeyCode::Char('c') => {
                                        // Clear logs
                                        app.clear_logs();
                                    }
                                    KeyCode::Char('f') => {
                                        // Cycle level filter
                                        app.cycle_level_filter();
                                    }
                                    KeyCode::Char('/') => {
                                        // Enter search mode
                                        app.enter_search_mode();
                                    }
                                    KeyCode::Char('s') => {
                                        // Save logs to file
                                        app.enter_save_mode();
                                    }
                                    KeyCode::Char('p') | KeyCode::Char(' ') => {
                                        // Toggle pause/resume
                                        app.toggle_pause();
                                    }
                                    KeyCode::Char('y') => {
                                        // Copy selected line
                                        if let Err(e) = app.copy_selected_line() {
                                            app.add_system_log(format!("Copy failed: {}", e));
                                        }
                                    }
                                    KeyCode::Char('?') => {
                                        // Toggle help
                                        app.toggle_help();
                                    }
                                    KeyCode::Up | KeyCode::Char('k') => {
                                        app.scroll_up(1);
                                    }
                                    KeyCode::Down | KeyCode::Char('j') => {
                                        app.scroll_down(1);
                                    }
                                    KeyCode::PageUp => {
                                        app.scroll_up(10);
                                    }
                                    KeyCode::PageDown => {
                                        app.scroll_down(10);
                                    }
                                    KeyCode::Home => {
                                        app.scroll_to_top();
                                    }
                                    KeyCode::End => {
                                        app.scroll_to_bottom();
                                    }
                                    _ => {}
                                }
                            }
                            InputMode::Search | InputMode::SavePrompt => {
                                match key.code {
                                    KeyCode::Enter => {
                                        if app.input_mode == InputMode::Search {
                                            app.confirm_search();
                                        } else if let Err(e) = app.save_logs() {
                                            app.add_system_log(format!("Save failed: {}", e));
                                        }
                                    }
                                    KeyCode::Esc => {
                                        app.exit_input_mode();
                                    }
                                    KeyCode::Backspace => {
                                        app.input_backspace();
                                    }
                                    KeyCode::Char(c) => {
                                        app.input_char(c);
                                    }
                                    _ => {}
                                }
                            }
                            InputMode::Help => {
                                // Any key closes help
                                app.toggle_help();
                            }
                        }
                    }
                    Event::Mouse(mouse) => {
                        use crossterm::event::MouseEventKind;
                        match mouse.kind {
                            MouseEventKind::ScrollUp => {
                                app.scroll_up(3);
                            }
                            MouseEventKind::ScrollDown => {
                                app.scroll_down(3);
                            }
                            MouseEventKind::Down(_) => {
                                // Clic pour sélectionner une ligne
                                app.select_line_at(mouse.row as usize);
                            }
                            _ => {}
                        }
                    }
                    Event::Tick => {
                        // Gérer le countdown
                        if let AppState::WaitingCountdown(n) = app.state {
                            if n <= 1 {
                                // Afficher "0s" puis quitter au prochain tick
                                if n == 1 {
                                    app.set_state(AppState::WaitingCountdown(0));
                                } else {
                                    app.should_quit = true;
                                }
                            } else {
                                app.set_state(AppState::WaitingCountdown(n - 1));
                            }
                        }
                        
                        // Vérifier si le processus est terminé
                        if let AppState::Running = app.state
                            && let Some(status) = supervisor.try_wait() {
                                app.set_pid(None);
                                if status.success() {
                                    app.add_system_log("Process exited successfully".to_string());
                                } else {
                                    app.add_system_log(format!("Process exited with status: {}", status));
                                }
                                
                                // Auto quit si configuré, sinon attendre 5 secondes
                                if config.agent.auto_quit {
                                    app.should_quit = true;
                                } else {
                                    app.set_state(AppState::WaitingCountdown(5));
                                }
                            }
                        
                        // Forcer un redraw périodique pour l'uptime
                        app.needs_redraw = true;
                    }
                    Event::Resize(_, _) => {
                        // Forcer un redraw complet
                        app.needs_redraw = true;
                    }
                }
            }
            
            // Nouveau log du processus
            Some(log) = channels.rx_log.recv() => {
                // Ajouter à l'affichage
                app.add_log(log.clone());
                
                // Envoyer au socket
                if channels.tx_socket.send(log).await.is_ok() {
                    app.increment_sent();
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
