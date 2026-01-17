use crate::tui::app::{App, AppState, InputMode};
use super::widgets::{ShortcutList, StatusInfoList};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Dessine la barre de statut avec infos et input selon le mode
pub fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    match app.input_mode {
        InputMode::Normal => draw_normal_status(frame, app, area),
        InputMode::Search => draw_search_input(frame, app, area),
        InputMode::SavePrompt => draw_save_input(frame, app, area),
        InputMode::Help => {}, // Géré par help_overlay
    }
}

/// Statut normal avec infos et raccourcis
fn draw_normal_status(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Contenu selon l'état
    let lines = match &app.state {
        AppState::Running => {
            let pid_str = app.pid.map_or("N/A".to_string(), |p| p.to_string());
            let scroll_str = if app.auto_scroll {
                "AUTO".to_string()
            } else {
                format!("{}/{}", app.logs.len().saturating_sub(app.scroll_offset), app.logs.len())
            };

            // Ligne 1: infos principales avec builder
            let info_spans = StatusInfoList::new()
                .add(" PID", pid_str, Color::Cyan)
                .add("Uptime", app.uptime(), Color::Green)
                .add("Scroll", scroll_str, Color::DarkGray)
                .add("Status", if app.paused { "PAUSED" } else { "LIVE" }, if app.paused { Color::Red } else { Color::Green })
                .add("Stats", format!("↓{} ↑{} {:.1}/s", app.total_logs_received, app.total_logs_sent, app.logs_per_second()), Color::Blue)
                .to_spans();

            let line1 = Line::from(info_spans);

            // Ligne 2: raccourcis avec builder
            let shortcut_spans = ShortcutList::new()
                .add("r", "Restart")
                .add("p", "Pause")
                .add("/", "Search")
                .add("s", "Save")
                .add("y", "Copy")
                .add("?", "Help")
                .add("q", "Quit")
                .to_spans();

            let mut line2_spans = vec![Span::raw(" ")];
            line2_spans.extend(shortcut_spans);
            let line2 = Line::from(line2_spans);

            vec![line1, line2]
        }
        AppState::WaitingCountdown(n) => {
            let shortcuts = ShortcutList::new()
                .add("r", "Restart")
                .add("q", "Quit")
                .to_spans();

            let mut spans = vec![
                Span::styled(" Process exited ", Style::default().fg(Color::Yellow)),
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            ];
            spans.extend(shortcuts);
            spans.extend(vec![
                Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("Auto-quit in {}s...", n),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]);

            vec![Line::from(spans)]
        }
        AppState::Restarting => {
            vec![Line::from(vec![Span::styled(
                " Restarting... ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )])]
        }
    };

    // Render les lignes
    render_lines(frame, inner_area, &lines);
}

/// Input de recherche
fn draw_search_input(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(Span::styled(
            " Search (regex) ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        // Ligne d'input
        Line::from(vec![
            Span::styled(" / ", Style::default().fg(Color::Yellow)),
            Span::styled(&app.input_buffer, Style::default().fg(Color::White)),
            Span::styled("█", Style::default().fg(Color::White)),
        ]),
        // Message ou aide
        Line::from(vec![Span::styled(
            format!(" {}", app.search_message.as_deref().unwrap_or("Enter to search, Esc to cancel")),
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    render_lines(frame, inner_area, &lines);
}

/// Input pour sauvegarder
fn draw_save_input(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title(Span::styled(
            " Save Logs ",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(vec![
            Span::styled(" Filename: ", Style::default().fg(Color::Green)),
            Span::styled(&app.input_buffer, Style::default().fg(Color::White)),
            Span::styled("█", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![Span::styled(
            " Enter to save, Esc to cancel ",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    render_lines(frame, inner_area, &lines);
}

/// Utilitaire pour render des lignes avec layout automatique
fn render_lines(frame: &mut Frame, area: Rect, lines: &[Line]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            lines
                .iter()
                .map(|_| Constraint::Length(1))
                .collect::<Vec<_>>(),
        )
        .split(area);

    for (i, line) in lines.iter().enumerate() {
        if i < chunks.len() {
            let paragraph = Paragraph::new(line.clone());
            frame.render_widget(paragraph, chunks[i]);
        }
    }
}
