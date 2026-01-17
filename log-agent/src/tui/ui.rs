use crate::tui::app::{App, AppState, LogLine};
use crate::types::LogLevel;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Dessine l'interface complète
pub fn draw(frame: &mut Frame, app: &mut App) {
    // Layout: header (1) + logs (flexible) + status bar (3)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Header
            Constraint::Min(1),     // Logs
            Constraint::Length(3),  // Status bar
        ])
        .split(frame.area());

    draw_header(frame, app, chunks[0]);
    draw_logs(frame, app, chunks[1]);
    draw_status_bar(frame, app, chunks[2]);
}

/// Dessine le header avec le projet et la commande
fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {} ", app.project),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            app.command_str(),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    
    frame.render_widget(header, area);
}

/// Dessine la zone des logs avec scrollbar
fn draw_logs(frame: &mut Frame, app: &mut App, area: Rect) {
    // Mettre à jour la hauteur visible
    app.visible_height = (area.height as usize).saturating_sub(2); // -2 pour les bordures

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            format!(" Logs ({}) ", app.logs.len()),
            Style::default().fg(Color::White),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Construire la liste des logs visibles
    let items: Vec<ListItem> = app
        .visible_logs()
        .map(|(idx, log)| {
            let is_selected = app.selected_line == Some(idx);
            log_to_list_item(log, is_selected)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner_area);

    // Scrollbar
    if app.logs.len() > app.visible_height {
        let scrollbar_area = Rect {
            x: area.x + area.width - 1,
            y: area.y + 1,
            width: 1,
            height: area.height - 2,
        };

        let total = app.logs.len();
        let position = total.saturating_sub(app.scroll_offset + app.visible_height);
        
        let mut scrollbar_state = ScrollbarState::new(total).position(position);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Convertit un log en ListItem avec couleurs
fn log_to_list_item(log: &LogLine, is_selected: bool) -> ListItem<'static> {
    let (level_str, level_color) = match log.level {
        LogLevel::Error => ("ERR", Color::Red),
        LogLevel::Warn => ("WRN", Color::Yellow),
        LogLevel::Debug => ("DBG", Color::Blue),
        LogLevel::Info => ("INF", Color::Green),
    };

    let base_style = if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };

    let line = if log.is_system {
        // Message système
        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(Color::DarkGray),
            ),
            Span::styled(
                "SYS ",
                base_style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                log.message.clone(),
                base_style.fg(Color::Magenta),
            ),
        ])
    } else {
        // Log normal
        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(Color::DarkGray),
            ),
            Span::styled(
                format!("{} ", level_str),
                base_style.fg(level_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                log.message.clone(),
                base_style.fg(match log.level {
                    LogLevel::Error => Color::Red,
                    LogLevel::Warn => Color::Yellow,
                    _ => Color::White,
                }),
            ),
        ])
    };

    ListItem::new(line)
}

/// Dessine la barre de statut
fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Contenu selon l'état
    let content = match &app.state {
        AppState::Running => {
            let pid_str = app.pid.map_or("N/A".to_string(), |p| p.to_string());
            let scroll_indicator = if app.auto_scroll {
                "AUTO".to_string()
            } else {
                format!("{}/{}", app.logs.len().saturating_sub(app.scroll_offset), app.logs.len())
            };
            
            Line::from(vec![
                Span::styled(" PID: ", Style::default().fg(Color::DarkGray)),
                Span::styled(pid_str, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled(app.uptime(), Style::default().fg(Color::Green)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled(scroll_indicator, Style::default().fg(Color::DarkGray)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled("[r]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" Restart ", Style::default().fg(Color::White)),
                Span::styled("[c]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" Clear ", Style::default().fg(Color::White)),
                Span::styled("[q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" Quit ", Style::default().fg(Color::White)),
            ])
        }
        AppState::WaitingCountdown(n) => {
            Line::from(vec![
                Span::styled(" Process exited ", Style::default().fg(Color::Yellow)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled("[r]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" Restart ", Style::default().fg(Color::White)),
                Span::styled("[q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" Quit ", Style::default().fg(Color::White)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("Auto-quit in {}s...", n),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ])
        }
        AppState::Restarting => {
            Line::from(vec![
                Span::styled(
                    " Restarting... ",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ])
        }
    };

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, inner_area);
}
