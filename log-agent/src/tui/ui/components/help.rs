use crate::tui::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Dessine l'overlay d'aide (popup centré)
pub fn draw_help_overlay(frame: &mut Frame, app: &App) {
    // Extract colors from config
    let border_color = app.config.performance.tui.colors.border.to_ratatui_color();
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
    let status_fg = app
        .config
        .performance
        .tui
        .colors
        .status_fg
        .to_ratatui_color();

    // Calculer la taille du popup (80% de la largeur, 90% de la hauteur)
    let area = frame.area();
    let popup_width = (area.width as f32 * 0.8) as u16;
    let popup_height = (area.height as f32 * 0.9) as u16;

    let popup_area = Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Ne pas utiliser Clear pour garder un effet de transparence
    // Utiliser un fond gris foncé avec une opacité visuelle
    // (Certains terminaux supportent cela mieux que d'autres)
    let transparent_bg = Color::Rgb(20, 20, 25); // Gris très foncé, presque noir

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            " Help - Keyboard Shortcuts (↑/↓ to scroll) ",
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(transparent_bg)); // Utiliser le fond semi-transparent

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Contenu de l'aide
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑/↓, j/k      ", Style::default().fg(status_fg)),
            Span::raw("Scroll up/down one line"),
        ]),
        Line::from(vec![
            Span::styled("  Page Up/Down  ", Style::default().fg(status_fg)),
            Span::raw("Scroll up/down 10 lines"),
        ]),
        Line::from(vec![
            Span::styled("  Home/End      ", Style::default().fg(status_fg)),
            Span::raw("Jump to top/bottom"),
        ]),
        Line::from(vec![
            Span::styled("  ←/→, h/l      ", Style::default().fg(status_fg)),
            Span::raw("Scroll horizontally (for long lines)"),
        ]),
        Line::from(vec![
            Span::styled("  0             ", Style::default().fg(status_fg)),
            Span::raw("Reset horizontal scroll (go to line start)"),
        ]),
        Line::from(vec![
            Span::styled("  Mouse Scroll  ", Style::default().fg(status_fg)),
            Span::raw("Scroll with mouse wheel"),
        ]),
        Line::from(vec![
            Span::styled("  Mouse Click   ", Style::default().fg(status_fg)),
            Span::raw("Select a log line"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Process Control",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  r             ", Style::default().fg(status_fg)),
            Span::raw("Restart process without quitting"),
        ]),
        Line::from(vec![
            Span::styled("  q             ", Style::default().fg(status_fg)),
            Span::raw("Quit the application"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Log Management",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  c             ", Style::default().fg(status_fg)),
            Span::raw("Clear all logs from view"),
        ]),
        Line::from(vec![
            Span::styled("  p / Space     ", Style::default().fg(status_fg)),
            Span::raw("Pause/Resume log capture"),
        ]),
        Line::from(vec![
            Span::styled("  /             ", Style::default().fg(status_fg)),
            Span::raw("Search logs (supports regex)"),
        ]),
        Line::from(vec![
            Span::styled("  s             ", Style::default().fg(status_fg)),
            Span::raw("Save logs to file"),
        ]),
        Line::from(vec![
            Span::styled("  y             ", Style::default().fg(status_fg)),
            Span::raw("Copy selected line to clipboard"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Bookmarks",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  m             ", Style::default().fg(status_fg)),
            Span::raw("Toggle bookmark on selected line (★)"),
        ]),
        Line::from(vec![
            Span::styled("  n             ", Style::default().fg(status_fg)),
            Span::raw("Jump to next bookmark"),
        ]),
        Line::from(vec![
            Span::styled("  N (Shift+n)   ", Style::default().fg(status_fg)),
            Span::raw("Jump to previous bookmark"),
        ]),
        Line::from(vec![
            Span::styled("  B (Shift+b)   ", Style::default().fg(status_fg)),
            Span::raw("Clear all bookmarks"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Filtering",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  f             ", Style::default().fg(status_fg)),
            Span::raw("Cycle log level filter (ALL/ERROR/WARN/INFO/DEBUG)"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Search Mode",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw("  When in search mode (/):")]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(search_dimmed)),
            Span::raw("Type regex pattern to filter logs"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(search_dimmed)),
            Span::raw("Matching logs are highlighted, others dimmed"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(search_dimmed)),
            Span::raw("Press Enter to apply, Esc to cancel"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(search_dimmed)),
            Span::raw("Enter empty search to clear filter"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Status Bar Info",
            Style::default()
                .fg(search_match)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  PID           ", Style::default().fg(status_fg)),
            Span::raw("Process ID of running command"),
        ]),
        Line::from(vec![
            Span::styled("  Uptime        ", Style::default().fg(status_fg)),
            Span::raw("Time since process started"),
        ]),
        Line::from(vec![
            Span::styled("  Scroll        ", Style::default().fg(status_fg)),
            Span::raw("Current position or AUTO if auto-scrolling"),
        ]),
        Line::from(vec![
            Span::styled("  LIVE/PAUSED   ", Style::default().fg(status_fg)),
            Span::raw("Capture status"),
        ]),
        Line::from(vec![
            Span::styled("  ↓↑ Stats      ", Style::default().fg(status_fg)),
            Span::raw("Logs received/sent and rate per second"),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Use ↑/↓ or j/k to scroll • Press any key to close",
            Style::default()
                .fg(search_dimmed)
                .add_modifier(Modifier::ITALIC),
        )]),
    ];

    // Appliquer le scroll
    let visible_height = inner_area.height as usize;
    let total_lines = help_text.len();
    let max_scroll = total_lines.saturating_sub(visible_height);

    // S'assurer que app.help_scroll ne dépasse pas max_scroll
    let scroll = app.help_scroll.min(max_scroll);

    // Prendre seulement les lignes visibles
    let visible_text: Vec<Line> = help_text
        .into_iter()
        .skip(scroll)
        .take(visible_height)
        .collect();

    let paragraph = Paragraph::new(visible_text)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, inner_area);
}
