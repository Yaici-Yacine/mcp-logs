use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

/// Structure pour un raccourci clavier formaté
#[derive(Debug, Clone)]
pub struct Shortcut {
    pub key: String,
    pub description: String,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: desc.into(),
        }
    }

    /// Convertit en spans pour affichage
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        vec![
            Span::styled(
                format!("[{}]", self.key),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {} ", self.description),
                Style::default().fg(Color::White),
            ),
        ]
    }
}

/// Builder pour créer une liste de raccourcis
pub struct ShortcutList(Vec<Shortcut>);

impl ShortcutList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(mut self, key: impl Into<String>, desc: impl Into<String>) -> Self {
        self.0.push(Shortcut::new(key, desc));
        self
    }

    /// Convertit tous les raccourcis en spans
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        self.0
            .iter()
            .flat_map(|shortcut| shortcut.to_spans())
            .collect()
    }
}

/// Structure pour une info de statut (label + valeur)
#[derive(Debug, Clone)]
pub struct StatusInfo {
    pub label: String,
    pub value: String,
    pub color: Color,
}

impl StatusInfo {
    pub fn new(label: impl Into<String>, value: impl Into<String>, color: Color) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            color,
        }
    }

    /// Convertit en spans
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        vec![
            Span::styled(
                format!("{}: ", self.label),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                self.value.clone(),
                Style::default().fg(self.color).add_modifier(Modifier::BOLD),
            ),
        ]
    }
}

/// Builder pour créer une liste d'infos avec séparateurs
pub struct StatusInfoList(Vec<StatusInfo>);

impl StatusInfoList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(mut self, label: impl Into<String>, value: impl Into<String>, color: Color) -> Self {
        self.0.push(StatusInfo::new(label, value, color));
        self
    }

    /// Convertit en spans avec séparateurs
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        let separator = Span::styled(" │ ", Style::default().fg(Color::DarkGray));

        for (i, info) in self.0.iter().enumerate() {
            if i > 0 {
                spans.push(separator.clone());
            }
            spans.extend(info.to_spans());
        }

        spans
    }
}
