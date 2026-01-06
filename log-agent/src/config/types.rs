use serde::{Deserialize, Serialize};

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
    pub verbose: bool,
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
#[serde(rename_all = "snake_case")]
pub enum Color {
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
}

fn default_buffer_size() -> usize {
    1000
}

fn default_flush_interval() -> u64 {
    100
}

// ==================== Default helpers for ColorStyle ====================

fn default_error_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Red),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_warn_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Yellow),
        bg: None,
        style: vec![],
    }
}

fn default_debug_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Blue),
        bg: None,
        style: vec![],
    }
}

fn default_info_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::White),
        bg: None,
        style: vec![],
    }
}

fn default_success_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Green),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_system_error_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Red),
        bg: None,
        style: vec![Style::Bold],
    }
}

fn default_system_info_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::Cyan),
        bg: None,
        style: vec![],
    }
}

fn default_dim_color() -> ColorStyle {
    ColorStyle {
        fg: Some(Color::BrightBlack),
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
            verbose: false,
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
                fg: Some(Color::Red),
                bg: None,
                style: vec![Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Yellow),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Blue),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::White),
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
                fg: Some(Color::Green),
                bg: None,
                style: vec![Style::Bold],
            },
            error: ColorStyle {
                fg: Some(Color::Red),
                bg: None,
                style: vec![Style::Bold],
            },
            info: ColorStyle {
                fg: Some(Color::Cyan),
                bg: None,
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::BrightBlack),
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
        }
    }
}
