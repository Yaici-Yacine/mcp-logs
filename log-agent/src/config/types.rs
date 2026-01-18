use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use owo_colors::OwoColorize;

impl Color {
    /// Convertit Color en ratatui::style::Color pour la TUI
    pub fn to_ratatui_color(&self) -> ratatui::style::Color {
        match self {
            Color::Hex(hex) => {
                // Parse hex color #RRGGBB ou RRGGBB
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        return ratatui::style::Color::Rgb(r, g, b);
                    }
                }
                // Fallback to white if invalid hex
                ratatui::style::Color::White
            }
            Color::Rgb(r, g, b) => ratatui::style::Color::Rgb(*r, *g, *b),
            Color::Named(name) => name.to_ratatui_color(),
        }
    }
    
    /// Applique la couleur à un texte avec owo_colors
    pub fn apply_to_string(&self, text: &str) -> String {
        match self {
            Color::Hex(hex) => {
                // Parse hex et utilise truecolor
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        return text.truecolor(r, g, b).to_string();
                    }
                }
                text.to_string()
            }
            Color::Rgb(r, g, b) => text.truecolor(*r, *g, *b).to_string(),
            Color::Named(name) => name.apply_to_string(text),
        }
    }
}

impl ColorName {
    /// Convertit ColorName en ratatui::style::Color
    pub fn to_ratatui_color(&self) -> ratatui::style::Color {
        match self {
            ColorName::Black => ratatui::style::Color::Black,
            ColorName::Red => ratatui::style::Color::Red,
            ColorName::Green => ratatui::style::Color::Green,
            ColorName::Yellow => ratatui::style::Color::Yellow,
            ColorName::Blue => ratatui::style::Color::Blue,
            ColorName::Magenta => ratatui::style::Color::Magenta,
            ColorName::Cyan => ratatui::style::Color::Cyan,
            ColorName::White => ratatui::style::Color::White,
            ColorName::BrightBlack => ratatui::style::Color::DarkGray,
            ColorName::BrightRed => ratatui::style::Color::LightRed,
            ColorName::BrightGreen => ratatui::style::Color::LightGreen,
            ColorName::BrightYellow => ratatui::style::Color::LightYellow,
            ColorName::BrightBlue => ratatui::style::Color::LightBlue,
            ColorName::BrightMagenta => ratatui::style::Color::LightMagenta,
            ColorName::BrightCyan => ratatui::style::Color::LightCyan,
            ColorName::BrightWhite => ratatui::style::Color::White,
        }
    }
    
    /// Applique la couleur à un texte avec owo_colors
    pub fn apply_to_string(&self, text: &str) -> String {
        match self {
            ColorName::Black => text.black().to_string(),
            ColorName::Red => text.red().to_string(),
            ColorName::Green => text.green().to_string(),
            ColorName::Yellow => text.yellow().to_string(),
            ColorName::Blue => text.blue().to_string(),
            ColorName::Magenta => text.magenta().to_string(),
            ColorName::Cyan => text.cyan().to_string(),
            ColorName::White => text.white().to_string(),
            ColorName::BrightBlack => text.bright_black().to_string(),
            ColorName::BrightRed => text.bright_red().to_string(),
            ColorName::BrightGreen => text.bright_green().to_string(),
            ColorName::BrightYellow => text.bright_yellow().to_string(),
            ColorName::BrightBlue => text.bright_blue().to_string(),
            ColorName::BrightMagenta => text.bright_magenta().to_string(),
            ColorName::BrightCyan => text.bright_cyan().to_string(),
            ColorName::BrightWhite => text.bright_white().to_string(),
        }
    }
}

/// Configuration d'une commande prédéfinie
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandConfig {
    /// Simple: juste la commande
    Simple(Vec<String>),
    /// Détaillée: commande + options
    Detailed {
        command: Vec<String>,
        #[serde(default)]
        watch: bool,
    },
}

/// Configuration principale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub agent: AgentConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub filters: FilterConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
}

/// Configuration de l'agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    #[serde(default = "default_socket_path")]
    pub socket_path: String,
    #[serde(default = "default_project_name")]
    pub default_project: String,
    #[serde(default)]
    pub default_command: Option<Vec<String>>,
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub watch: bool,
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: u32,
}

fn default_socket_path() -> String {
    "/tmp/log-agent.sock".to_string()
}

fn default_project_name() -> String {
    "default".to_string()
}

fn default_connection_timeout() -> u64 {
    5
}

fn default_retry_attempts() -> u32 {
    3
}

/// Configuration de la sortie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_true")]
    pub colors: bool,
    #[serde(default)]
    pub format: OutputFormat,
    #[serde(default)]
    pub show_timestamps: bool,
    #[serde(default)]
    pub show_pid: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Colored,
    Plain,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Colored
    }
}

/// Configuration des couleurs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    #[serde(default = "default_error_color")]
    pub error: ColorStyle,
    #[serde(default = "default_warn_color")]
    pub warn: ColorStyle,
    #[serde(default = "default_debug_color")]
    pub debug: ColorStyle,
    #[serde(default = "default_info_color")]
    pub info: ColorStyle,
    #[serde(default)]
    pub system: SystemColorConfig,
}

/// Configuration des couleurs système (messages du CLI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemColorConfig {
    #[serde(default = "default_success_color")]
    pub success: ColorStyle,
    #[serde(default = "default_system_error_color")]
    pub error: ColorStyle,
    #[serde(default = "default_system_info_color")]
    pub info: ColorStyle,
    #[serde(default = "default_dim_color")]
    pub dim: ColorStyle,
}

/// Style de couleur (foreground, background, styles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    #[serde(default)]
    pub style: Vec<Style>,
}

/// Couleurs disponibles
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    /// Couleur hexadécimale (ex: "#FF5733" ou "FF5733")
    Hex(String),
    /// Couleur RGB
    Rgb(u8, u8, u8),
    /// Couleur nommée
    Named(ColorName),
}

/// Noms de couleurs standards
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Styles de texte
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Strikethrough,
}

/// Configuration des filtres
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
    #[serde(default)]
    pub min_level: LogLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Debug
    }
}

/// Configuration des performances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
    #[serde(default = "default_flush_interval")]
    pub flush_interval: u64,
    #[serde(default)]
    pub tui: TuiConfig,
}

/// Configuration de la TUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    #[serde(default = "default_max_logs")]
    pub max_logs: usize,
    #[serde(default = "default_tick_rate")]
    pub tick_rate_ms: u64,
    #[serde(default = "default_frame_rate")]
    pub frame_rate_ms: u64,
    #[serde(default)]
    pub colors: TuiColorConfig,
}

/// Configuration des couleurs de la TUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiColorConfig {
    #[serde(default = "default_tui_header_bg")]
    pub header_bg: Color,
    #[serde(default = "default_tui_header_fg")]
    pub header_fg: Color,
    #[serde(default = "default_tui_status_bg")]
    pub status_bg: Color,
    #[serde(default = "default_tui_status_fg")]
    pub status_fg: Color,
    #[serde(default = "default_tui_border")]
    pub border: Color,
    #[serde(default = "default_tui_selected_bg")]
    pub selected_bg: Color,
    #[serde(default = "default_tui_selected_fg")]
    pub selected_fg: Color,
    #[serde(default = "default_tui_search_match")]
    pub search_match: Color,
    #[serde(default = "default_tui_search_dimmed")]
    pub search_dimmed: Color,
    #[serde(default = "default_tui_help_bg")]
    pub help_bg: Color,
    #[serde(default = "default_tui_help_fg")]
    pub help_fg: Color,
}

fn default_max_logs() -> usize {
    5000
}

fn default_tick_rate() -> u64 {
    250  // 250ms entre les ticks (countdown)
}

fn default_frame_rate() -> u64 {
    100  // 100ms entre les frames (10 FPS pour réduire les lags)
}

fn default_buffer_size() -> usize {
    1000
}

fn default_flush_interval() -> u64 {
    100
}

// ==================== Default helpers for TUI colors ====================

fn default_tui_header_bg() -> Color {
    Color::Named(ColorName::Blue)
}

fn default_tui_header_fg() -> Color {
    Color::Named(ColorName::White)
}

fn default_tui_status_bg() -> Color {
    Color::Named(ColorName::Black)
}

fn default_tui_status_fg() -> Color {
    Color::Named(ColorName::Cyan)
}

fn default_tui_border() -> Color {
    Color::Named(ColorName::Cyan)
}

fn default_tui_selected_bg() -> Color {
    Color::Named(ColorName::Cyan)
}

fn default_tui_selected_fg() -> Color {
    Color::Named(ColorName::Black)
}

fn default_tui_search_match() -> Color {
    Color::Named(ColorName::Yellow)
}

fn default_tui_search_dimmed() -> Color {
    Color::Named(ColorName::BrightBlack)
}

fn default_tui_help_bg() -> Color {
    Color::Named(ColorName::Black)
}

fn default_tui_help_fg() -> Color {
    Color::Named(ColorName::White)
}

// ==================== Default helpers for ColorStyle ====================

fn default_error_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Red)),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_warn_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Yellow)),
        bg: None,
        style: vec![],
    }
}

fn default_debug_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Blue)),
        bg: None,
        style: vec![],
    }
}

fn default_info_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::White)),
        bg: None,
        style: vec![],
    }
}

fn default_success_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Green)),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_system_error_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Red)),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_system_info_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::Cyan)),
        bg: None,
        style: vec![],
    }
}

fn default_dim_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Named(ColorName::BrightBlack)),
        bg: None,
        style: vec![],
    }
}

// ==================== Implémentations Default ====================

impl Default for Config {
    fn default() -> Self {
        Self {
            agent: AgentConfig::default(),
            output: OutputConfig::default(),
            colors: ColorConfig::default(),
            filters: FilterConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            socket_path: "/tmp/log-agent.sock".to_string(),
            default_project: "default".to_string(),
            default_command: None,
            commands: HashMap::new(),
            verbose: false,
            watch: false,
            connection_timeout: 5,
            retry_attempts: 3,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            colors: true,
            format: OutputFormat::Colored,
            show_timestamps: false,
            show_pid: false,
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::Red)),
                bg: None,
                style: vec![Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Named(ColorName::Yellow)),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Named(ColorName::Blue)),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig::default(),
        }
    }
}

impl Default for SystemColorConfig {
    fn default() -> Self {
        Self {
            success: ColorStyle {
                fg: Some(Color::Named(ColorName::Green)),
                bg: None,
                style: vec![Style::Bold],
            },
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::Red)),
                bg: None,
                style: vec![Style::Bold],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::Cyan)),
                bg: None,
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: None,
                style: vec![],
            },
        }
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            ignore_patterns: vec![],
            min_level: LogLevel::Debug,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            flush_interval: 100,
            tui: TuiConfig::default(),
        }
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            max_logs: 5000,
            tick_rate_ms: 250,
            frame_rate_ms: 100,
            colors: TuiColorConfig::default(),
        }
    }
}

impl Default for TuiColorConfig {
    fn default() -> Self {
        Self {
            header_bg: default_tui_header_bg(),
            header_fg: default_tui_header_fg(),
            status_bg: default_tui_status_bg(),
            status_fg: default_tui_status_fg(),
            border: default_tui_border(),
            selected_bg: default_tui_selected_bg(),
            selected_fg: default_tui_selected_fg(),
            search_match: default_tui_search_match(),
            search_dimmed: default_tui_search_dimmed(),
            help_bg: default_tui_help_bg(),
            help_fg: default_tui_help_fg(),
        }
    }
}
