mod components;

use crate::tui::app::{App, InputMode};
use components::*;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

/// Dessine l'interface complète
pub fn draw(frame: &mut Frame, app: &mut App) {
    // Si mode aide, afficher l'overlay d'aide
    if app.input_mode == InputMode::Help {
        draw_help_overlay(frame, app);
        return;
    }

    // Layout principal: header (1) + logs (flexible) + status bar (3 ou 4)
    let status_height = if app.input_mode != InputMode::Normal { 4 } else { 3 };
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),            // Header
            Constraint::Min(1),               // Logs
            Constraint::Length(status_height), // Status bar (+ input si nécessaire)
        ])
        .split(frame.area());

    draw_header(frame, app, chunks[0]);
    draw_logs_panel(frame, app, chunks[1]);
    draw_status_bar(frame, app, chunks[2]);
}
