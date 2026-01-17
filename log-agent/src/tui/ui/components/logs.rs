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
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(title, Style::default().fg(Color::White)));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Construire la liste des logs visibles avec filtrage
    let items: Vec<ListItem> = app
        .filtered_visible_logs()
        .iter()
        .map(|(idx, log, matches)| {
            let is_selected = app.selected_line == Some(*idx);
            log_to_list_item(log, is_selected, *matches)
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
fn log_to_list_item(log: &LogLine, is_selected: bool, matches_filter: bool) -> ListItem<'static> {
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

    // Si le log ne matche pas le filtre, l'afficher en grisé
    let dimmed = !matches_filter;

    let line = if log.is_system {
        // Message système
        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(Color::DarkGray),
            ),
            Span::styled(
                "SYS ",
                base_style
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(log.message.clone(), base_style.fg(Color::Magenta)),
        ])
    } else {
        // Log normal
        let msg_color = if dimmed {
            Color::DarkGray
        } else {
            match log.level {
                LogLevel::Error => Color::Red,
                LogLevel::Warn => Color::Yellow,
                _ => Color::White,
            }
        };

        Line::from(vec![
            Span::styled(
                format!("{} ", log.timestamp),
                base_style.fg(if dimmed {
                    Color::DarkGray
                } else {
                    Color::DarkGray
                }),
            ),
            Span::styled(
                format!("{} ", level_str),
                base_style
                    .fg(if dimmed { Color::DarkGray } else { level_color })
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
