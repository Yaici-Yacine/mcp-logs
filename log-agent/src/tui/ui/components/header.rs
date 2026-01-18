use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Dessine le header avec le projet et la commande
pub fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    // Récupérer les couleurs de la config
    let header_bg = app.config.performance.tui.colors.header_bg.to_ratatui_color();
    let header_fg = app.config.performance.tui.colors.header_fg.to_ratatui_color();
    let status_fg = app.config.performance.tui.colors.status_fg.to_ratatui_color();
    
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {} ", app.project),
            Style::default()
                .fg(header_fg)
                .bg(header_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(app.command_str(), Style::default().fg(status_fg)),
    ]));

    frame.render_widget(header, area);
}
