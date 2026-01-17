use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Dessine l'overlay d'aide (popup centré)
pub fn draw_help_overlay(frame: &mut Frame, _app: &App) {
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

    // Effacer l'arrière-plan
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            " Help - Keyboard Shortcuts ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Contenu de l'aide
    let help_text = vec![
        Line::from(vec![
            Span::styled("Navigation", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑/↓, j/k      ", Style::default().fg(Color::Green)),
            Span::raw("Scroll up/down one line"),
        ]),
        Line::from(vec![
            Span::styled("  Page Up/Down  ", Style::default().fg(Color::Green)),
            Span::raw("Scroll up/down 10 lines"),
        ]),
        Line::from(vec![
            Span::styled("  Home/End      ", Style::default().fg(Color::Green)),
            Span::raw("Jump to top/bottom"),
        ]),
        Line::from(vec![
            Span::styled("  Mouse Scroll  ", Style::default().fg(Color::Green)),
            Span::raw("Scroll with mouse wheel"),
        ]),
        Line::from(vec![
            Span::styled("  Mouse Click   ", Style::default().fg(Color::Green)),
            Span::raw("Select a log line"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Process Control", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  r             ", Style::default().fg(Color::Green)),
            Span::raw("Restart process without quitting"),
        ]),
        Line::from(vec![
            Span::styled("  q             ", Style::default().fg(Color::Green)),
            Span::raw("Quit the application"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Log Management", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  c             ", Style::default().fg(Color::Green)),
            Span::raw("Clear all logs from view"),
        ]),
        Line::from(vec![
            Span::styled("  p / Space     ", Style::default().fg(Color::Green)),
            Span::raw("Pause/Resume log capture"),
        ]),
        Line::from(vec![
            Span::styled("  /             ", Style::default().fg(Color::Green)),
            Span::raw("Search logs (supports regex)"),
        ]),
        Line::from(vec![
            Span::styled("  s             ", Style::default().fg(Color::Green)),
            Span::raw("Save logs to file"),
        ]),
        Line::from(vec![
            Span::styled("  y             ", Style::default().fg(Color::Green)),
            Span::raw("Copy selected line to clipboard"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Search Mode", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  When in search mode (/):"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::DarkGray)),
            Span::raw("Type regex pattern to filter logs"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::DarkGray)),
            Span::raw("Matching logs are highlighted, others dimmed"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::DarkGray)),
            Span::raw("Press Enter to apply, Esc to cancel"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::DarkGray)),
            Span::raw("Enter empty search to clear filter"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Status Bar Info", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  PID           ", Style::default().fg(Color::Green)),
            Span::raw("Process ID of running command"),
        ]),
        Line::from(vec![
            Span::styled("  Uptime        ", Style::default().fg(Color::Green)),
            Span::raw("Time since process started"),
        ]),
        Line::from(vec![
            Span::styled("  Scroll        ", Style::default().fg(Color::Green)),
            Span::raw("Current position or AUTO if auto-scrolling"),
        ]),
        Line::from(vec![
            Span::styled("  LIVE/PAUSED   ", Style::default().fg(Color::Green)),
            Span::raw("Capture status"),
        ]),
        Line::from(vec![
            Span::styled("  ↓↑ Stats      ", Style::default().fg(Color::Green)),
            Span::raw("Logs received/sent and rate per second"),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press any key to close this help", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, inner_area);
}
