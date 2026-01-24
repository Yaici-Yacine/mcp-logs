use super::widgets::{ShortcutList, StatusInfoList};
use crate::tui::app::{App, AppState, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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
        InputMode::Help => {} // Géré par help_overlay
    }
}

/// Statut normal avec infos et raccourcis
fn draw_normal_status(frame: &mut Frame, app: &App, area: Rect) {
    // Récupérer les couleurs de la config
    let border_color = app.config.performance.tui.colors.border.to_ratatui_color();
    let status_fg = app
        .config
        .performance
        .tui
        .colors
        .status_fg
        .to_ratatui_color();
    let search_match = app
        .config
        .performance
        .tui
        .colors
        .search_match
        .to_ratatui_color();
    let search_dimmed = app
        .config
        .performance
        .tui
        .colors
        .search_dimmed
        .to_ratatui_color();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Contenu selon l'état
    let lines = match &app.state {
        AppState::Running => {
            let pid_str = app.pid.map_or("N/A".to_string(), |p| p.to_string());
            let scroll_str = if app.auto_scroll {
                "AUTO".to_string()
            } else {
                format!(
                    "{}/{}",
                    app.logs.len().saturating_sub(app.scroll_offset),
                    app.logs.len()
                )
            };

            // Ligne 1: infos principales avec builder
            let filter_label = app.level_filter.label();
            let filter_text = if matches!(app.level_filter, crate::tui::app::LevelFilter::All) {
                filter_label.to_string()
            } else {
                format!("{}!", filter_label) // Ajouter ! pour indiquer un filtre actif
            };

            let info_spans = StatusInfoList::new()
                .add(" PID", pid_str, status_fg)
                .add("Uptime", app.uptime(), status_fg)
                .add(
                    "Filter",
                    filter_text,
                    if matches!(app.level_filter, crate::tui::app::LevelFilter::All) {
                        status_fg
                    } else {
                        search_match
                    },
                )
                .add("Scroll", scroll_str, search_dimmed)
                .add(
                    "Status",
                    if app.paused { "PAUSED" } else { "LIVE" },
                    if app.paused { search_match } else { status_fg },
                )
                .add(
                    "Stats",
                    format!(
                        "↓{} ↑{} {:.1}/s",
                        app.total_logs_received,
                        app.total_logs_sent,
                        app.logs_per_second()
                    ),
                    status_fg,
                )
                .to_spans();

            let line1 = Line::from(info_spans);

            // Ligne 2: raccourcis avec builder
            let shortcut_spans = ShortcutList::new()
                .add("r", "Restart")
                .add("p", "Pause")
                .add("f", "Filter")
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
                Span::styled(" Process exited ", Style::default().fg(search_match)),
                Span::styled(" │ ", Style::default().fg(search_dimmed)),
            ];
            spans.extend(shortcuts);
            spans.extend(vec![
                Span::styled(" │ ", Style::default().fg(search_dimmed)),
                Span::styled(
                    format!("Auto-quit in {}s...", n),
                    Style::default()
                        .fg(search_match)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);

            vec![Line::from(spans)]
        }
        AppState::Restarting => {
            vec![Line::from(vec![Span::styled(
                " Restarting... ",
                Style::default().fg(status_fg).add_modifier(Modifier::BOLD),
            )])]
        }
    };

    // Render les lignes
    render_lines(frame, inner_area, &lines);
}

/// Input de recherche
fn draw_search_input(frame: &mut Frame, app: &App, area: Rect) {
    // Récupérer les couleurs de la config
    let search_match = app
        .config
        .performance
        .tui
        .colors
        .search_match
        .to_ratatui_color();
    let search_dimmed = app
        .config
        .performance
        .tui
        .colors
        .search_dimmed
        .to_ratatui_color();
    let help_fg = app.config.performance.tui.colors.help_fg.to_ratatui_color();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(search_match))
        .title(Span::styled(
            " Search (regex) ",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        // Ligne d'input
        Line::from(vec![
            Span::styled(" / ", Style::default().fg(search_match)),
            Span::styled(&app.input_buffer, Style::default().fg(help_fg)),
            Span::styled("█", Style::default().fg(help_fg)),
        ]),
        // Message ou aide
        Line::from(vec![Span::styled(
            format!(
                " {}",
                app.search_message
                    .as_deref()
                    .unwrap_or("Enter to search, Esc to cancel")
            ),
            Style::default().fg(search_dimmed),
        )]),
    ];

    render_lines(frame, inner_area, &lines);
}

/// Input pour sauvegarder
fn draw_save_input(frame: &mut Frame, app: &App, area: Rect) {
    // Récupérer les couleurs de la config
    let status_fg = app
        .config
        .performance
        .tui
        .colors
        .status_fg
        .to_ratatui_color();
    let search_dimmed = app
        .config
        .performance
        .tui
        .colors
        .search_dimmed
        .to_ratatui_color();
    let help_fg = app.config.performance.tui.colors.help_fg.to_ratatui_color();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(status_fg))
        .title(Span::styled(
            " Save Logs ",
            Style::default().fg(status_fg).add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(vec![
            Span::styled(" Filename: ", Style::default().fg(status_fg)),
            Span::styled(&app.input_buffer, Style::default().fg(help_fg)),
            Span::styled("█", Style::default().fg(help_fg)),
        ]),
        Line::from(vec![Span::styled(
            " Enter to save, Esc to cancel ",
            Style::default().fg(search_dimmed),
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
