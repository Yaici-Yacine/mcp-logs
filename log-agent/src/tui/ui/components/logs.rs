use crate::tui::app::{App, LogLine};
use crate::types::LogLevel;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Dessine la zone des logs avec scrollbar et filtrage
pub fn draw_logs_panel(frame: &mut Frame, app: &mut App, area: Rect) {
    // Extract colors from config
    let border_color = app.config.performance.tui.colors.border.to_ratatui_color();
    let header_fg = app.config.performance.tui.colors.header_fg.to_ratatui_color();

    // Mettre à jour la hauteur visible
    app.visible_height = (area.height as usize).saturating_sub(2); // -2 pour les bordures

    // Titre avec infos de filtrage
    let title = if app.search_regex.is_some() {
        format!(" Logs ({}/{}) - Filtered ", app.filtered_count(), app.logs.len())
    } else {
        format!(" Logs ({}) ", app.logs.len())
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(title, Style::default().fg(header_fg)));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Construire la liste des logs visibles avec filtrage
    let items: Vec<ListItem> = app
        .filtered_visible_logs()
        .iter()
        .map(|(idx, log, matches)| {
            let is_selected = app.selected_line == Some(*idx);
            log_to_list_item(log, is_selected, *matches, app)
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

/// Convertit un log en ListItem avec couleurs et surbrillance de recherche
fn log_to_list_item(log: &LogLine, is_selected: bool, matches_filter: bool, app: &App) -> ListItem<'static> {
    // Extract colors from config
    let selected_bg = app.config.performance.tui.colors.selected_bg.to_ratatui_color();
    let search_dimmed = app.config.performance.tui.colors.search_dimmed.to_ratatui_color();
    
    // Get log level colors from config (with fallbacks)
    let error_color = app.config.colors.error.fg
        .as_ref()
        .map(|c| c.to_ratatui_color())
        .unwrap_or(Color::Red);
    let warn_color = app.config.colors.warn.fg
        .as_ref()
        .map(|c| c.to_ratatui_color())
        .unwrap_or(Color::Yellow);
    let info_color = app.config.colors.info.fg
        .as_ref()
        .map(|c| c.to_ratatui_color())
        .unwrap_or(Color::Green);
    let debug_color = app.config.colors.debug.fg
        .as_ref()
        .map(|c| c.to_ratatui_color())
        .unwrap_or(Color::Blue);

    let (level_str, level_color) = match log.level {
        LogLevel::Error => ("ERR", error_color),
        LogLevel::Warn => ("WRN", warn_color),
        LogLevel::Debug => ("DBG", debug_color),
        LogLevel::Info => ("INF", info_color),
    };

    let base_style = if is_selected {
        Style::default().bg(selected_bg)
    } else {
        Style::default()
    };

    // Si le log ne matche pas le filtre, l'afficher en grisé
    let dimmed = !matches_filter;

    let line = if log.is_system {
        // Message système - use a magenta/purple color
        let system_color = Color::Magenta;
        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(search_dimmed),
            ),
            Span::styled(
                "SYS ",
                base_style
                    .fg(system_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(log.message.clone(), base_style.fg(system_color)),
        ])
    } else {
        // Log normal
        let msg_color = if dimmed {
            search_dimmed
        } else {
            match log.level {
                LogLevel::Error => error_color,
                LogLevel::Warn => warn_color,
                _ => Color::White,
            }
        };

        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(search_dimmed),
            ),
            Span::styled(
                format!("{} ", level_str),
                base_style
                    .fg(if dimmed { search_dimmed } else { level_color })
                    .add_modifier(if dimmed {
                        Modifier::empty()
                    } else {
                        Modifier::BOLD
                    }),
            ),
            Span::styled(log.message.clone(), base_style.fg(msg_color)),
        ])
    };

    ListItem::new(line)
}
