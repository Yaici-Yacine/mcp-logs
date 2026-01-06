use super::types::Config;
use std::fs;
use std::path::PathBuf;

/// Charge la configuration depuis toutes les sources et fusionne
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut merged_table = toml::Table::new();

    // 1. Charger config globale si elle existe et merger
    if let Some(global_path) = get_global_config_path() {
        if global_path.exists() {
            let contents = fs::read_to_string(&global_path)?;
            let global_table: toml::Table = toml::from_str(&contents)?;
            merge_toml_tables(&mut merged_table, global_table);
        }
    }

    // 2. Charger config locale si elle existe et merger
    let local_path = get_local_config_path();
    if local_path.exists() {
        let contents = fs::read_to_string(&local_path)?;
        let local_table: toml::Table = toml::from_str(&contents)?;
        merge_toml_tables(&mut merged_table, local_table);
    }

    // 3. Désérialiser en Config (avec defaults pour les champs manquants)
    let mut config: Config = toml::Value::Table(merged_table).try_into()
        .unwrap_or_else(|_| Config::default());

    // 4. Appliquer variables d'environnement
    config = apply_env_vars(config);

    Ok(config)
}

/// Merge two toml tables recursively
fn merge_toml_tables(base: &mut toml::Table, override_table: toml::Table) {
    for (key, value) in override_table {
        if let Some(base_val) = base.get(&key) {
            // Si les deux sont des tables, merger récursivement
            if base_val.is_table() && value.is_table() {
                let mut base_subtable = base_val.as_table().unwrap().clone();
                let override_subtable = value.as_table().unwrap().clone();
                merge_toml_tables(&mut base_subtable, override_subtable);
                base.insert(key, toml::Value::Table(base_subtable));
            } else {
                // Sinon remplacer directement
                base.insert(key, value);
            }
        } else {
            // Nouvelle clé, l'ajouter
            base.insert(key, value);
        }
    }
}

/// Applique les variables d'environnement à la config
fn apply_env_vars(mut config: Config) -> Config {
    // Agent
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_SOCKET_PATH") {
        config.agent.socket_path = val;
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_DEFAULT_PROJECT") {
        config.agent.default_project = val;
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_VERBOSE") {
        config.agent.verbose = val.to_lowercase() == "true";
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_CONNECTION_TIMEOUT") {
        if let Ok(timeout) = val.parse() {
            config.agent.connection_timeout = timeout;
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_RETRY_ATTEMPTS") {
        if let Ok(attempts) = val.parse() {
            config.agent.retry_attempts = attempts;
        }
    }

    // Output
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_COLORS") {
        config.output.colors = val.to_lowercase() == "true";
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_FORMAT") {
        match val.to_lowercase().as_str() {
            "colored" => config.output.format = super::types::OutputFormat::Colored,
            "plain" => config.output.format = super::types::OutputFormat::Plain,
            "json" => config.output.format = super::types::OutputFormat::Json,
            _ => {}
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_SHOW_TIMESTAMPS") {
        config.output.show_timestamps = val.to_lowercase() == "true";
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_SHOW_PID") {
        config.output.show_pid = val.to_lowercase() == "true";
    }

    // Performance
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_BUFFER_SIZE") {
        if let Ok(size) = val.parse() {
            config.performance.buffer_size = size;
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_FLUSH_INTERVAL") {
        if let Ok(interval) = val.parse() {
            config.performance.flush_interval = interval;
        }
    }

    // Filters
    if let Ok(val) = std::env::var("MCP_LOG_FILTER_MIN_LEVEL") {
        match val.to_lowercase().as_str() {
            "debug" => config.filters.min_level = super::types::LogLevel::Debug,
            "info" => config.filters.min_level = super::types::LogLevel::Info,
            "warn" => config.filters.min_level = super::types::LogLevel::Warn,
            "error" => config.filters.min_level = super::types::LogLevel::Error,
            _ => {}
        }
    }
    // Note: ignore_patterns serait complexe via env vars (liste), skip pour l'instant

    // Colors - Pour les couleurs via env vars, on supporte les couleurs fg principales
    // Format: MCP_LOG_COLOR_ERROR_FG=bright_red, MCP_LOG_COLOR_WARN_FG=yellow, etc.
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_ERROR_FG") {
        if let Some(color) = parse_color(&val) {
            config.colors.error.fg = Some(color);
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_WARN_FG") {
        if let Some(color) = parse_color(&val) {
            config.colors.warn.fg = Some(color);
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_INFO_FG") {
        if let Some(color) = parse_color(&val) {
            config.colors.info.fg = Some(color);
        }
    }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_DEBUG_FG") {
        if let Some(color) = parse_color(&val) {
            config.colors.debug.fg = Some(color);
        }
    }

    config
}

/// Parse une couleur depuis une string
fn parse_color(s: &str) -> Option<super::types::Color> {
    use super::types::Color;
    match s.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "bright_black" => Some(Color::BrightBlack),
        "bright_red" => Some(Color::BrightRed),
        "bright_green" => Some(Color::BrightGreen),
        "bright_yellow" => Some(Color::BrightYellow),
        "bright_blue" => Some(Color::BrightBlue),
        "bright_magenta" => Some(Color::BrightMagenta),
        "bright_cyan" => Some(Color::BrightCyan),
        "bright_white" => Some(Color::BrightWhite),
        _ => None,
    }
}

/// Retourne le chemin de la config globale
pub fn get_global_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("mcp-log-agent");
        path.push("config.toml");
        path
    })
}

/// Retourne le chemin de la config locale
pub fn get_local_config_path() -> PathBuf {
    PathBuf::from(".mcp-log-agent.toml")
}

/// Crée une config par défaut dans un fichier avec des commentaires explicatifs ligne par ligne
pub fn create_default_config(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Au lieu d'utiliser toml::to_string, on construit manuellement avec des commentaires détaillés
    let commented_toml = r#"# MCP Log Agent Configuration File
# Configuration priority (highest to lowest):
#   1. CLI arguments
#   2. Environment variables (MCP_LOG_*)
#   3. Local config file (.mcp-log-agent.toml)
#   4. Global config file (~/.config/mcp-log-agent/config.toml)
#   5. Default values

# ============================================================================
# [agent] - Core agent settings
# ============================================================================
[agent]

# socket_path: Path to the Unix socket for communication with mcp-logs server
# Default: "/tmp/log-agent.sock"
# Env var: MCP_LOG_AGENT_SOCKET_PATH
socket_path = "/tmp/log-agent.sock"

# default_project: Default project name when --project flag is not specified
# Default: "default"
# Env var: MCP_LOG_AGENT_DEFAULT_PROJECT
default_project = "default"

# default_command: Default command to run when no command is provided
# This allows you to simply run "mcp-log-agent run" without arguments
# Default: none
# Example: default_command = ["npm", "start"]
# Example: default_command = ["bun", "dev"]
# Example: default_command = ["cargo", "run", "--release"]
# Uncomment and modify to use:
# default_command = ["npm", "start"]

# verbose: Enable verbose logging output from the agent itself
# Default: false
# Possible values: true, false
# Env var: MCP_LOG_AGENT_VERBOSE
verbose = false

# connection_timeout: Socket connection timeout in seconds
# Default: 5
# Env var: MCP_LOG_AGENT_CONNECTION_TIMEOUT
connection_timeout = 5

# retry_attempts: Number of connection retry attempts on failure
# Default: 3
# Env var: MCP_LOG_AGENT_RETRY_ATTEMPTS
retry_attempts = 3

# ============================================================================
# [output] - Output formatting and display settings
# ============================================================================
[output]

# colors: Enable/disable ANSI color codes in terminal output
# Default: true
# Possible values: true, false
# Env var: MCP_LOG_OUTPUT_COLORS
colors = true

# format: Output format style
# Default: "colored"
# Possible values: "colored", "plain", "json"
# Env var: MCP_LOG_OUTPUT_FORMAT
format = "colored"

# show_timestamps: Display timestamps in the output
# Default: false
# Possible values: true, false
# Env var: MCP_LOG_OUTPUT_SHOW_TIMESTAMPS
show_timestamps = false

# show_pid: Display process ID in the output
# Default: false
# Possible values: true, false
# Env var: MCP_LOG_OUTPUT_SHOW_PID
show_pid = false

# ============================================================================
# [colors] - Color customization for different log levels
# ============================================================================
# Available colors: black, red, green, yellow, blue, magenta, cyan, white,
#                   bright_black, bright_red, bright_green, bright_yellow,
#                   bright_blue, bright_magenta, bright_cyan, bright_white
# Available styles: bold, italic, underline, dimmed, blink, reverse, strikethrough
#
# Quick start with predefined schemes:
#   mcp-log-agent config colors list             # Show available schemes
#   mcp-log-agent config colors set <name>       # Apply a scheme
#   mcp-log-agent config colors preview <name>   # Preview before applying
# Available schemes: default, solarized-dark, high-contrast, minimal, monochrome

# Error level colors (for ERROR logs and stderr)
[colors.error]
# fg: Foreground (text) color for error messages
fg = "red"
# style: Text style modifiers (can combine multiple: ["bold", "italic"])
style = ["bold"]

# Warning level colors (for WARN logs)
[colors.warn]
fg = "yellow"
style = []

# Debug level colors (for DEBUG logs)
[colors.debug]
fg = "blue"
style = []

# Info level colors (for INFO logs and stdout)
[colors.info]
fg = "white"
style = []

# System message colors (agent's own output, not from captured process)
[colors.system.success]
# Success messages like "✓ Process started"
fg = "green"
style = ["bold"]

[colors.system.error]
# Error messages from the agent itself
fg = "red"
style = ["bold"]

[colors.system.info]
# Informational messages like connection status
fg = "cyan"
style = []

[colors.system.dim]
# Secondary/less important information
fg = "bright_black"
style = []

# ============================================================================
# [filters] - Log filtering and level control
# ============================================================================
[filters]

# ignore_patterns: List of regex patterns to exclude from logs
# Default: []
# Example: ["^DEBUG:", "node_modules", "webpack"]
ignore_patterns = []

# min_level: Minimum log level to capture
# Default: "debug"
# Possible values: "debug", "info", "warn", "error"
# Note: "debug" captures all levels, "error" captures only errors
min_level = "debug"

# ============================================================================
# [performance] - Performance tuning settings
# ============================================================================
[performance]

# buffer_size: Internal buffer size for log messages
# Default: 1000
# Higher values use more memory but handle bursts better
buffer_size = 1000

# flush_interval: Interval in milliseconds to flush buffered logs
# Default: 100
# Lower values reduce latency but increase overhead
flush_interval = 100
"#;
    
    // Créer le répertoire parent si nécessaire
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, commented_toml)?;
    Ok(())
}

/// Sauvegarde une config dans un fichier
pub fn save_config(config: &Config, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string_pretty(config)?;
    
    // Créer le répertoire parent si nécessaire
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, toml_string)?;
    Ok(())
}

/// Vérifie si une config locale existe
pub fn has_local_config() -> bool {
    get_local_config_path().exists()
}

/// Vérifie si une config globale existe
pub fn has_global_config() -> bool {
    get_global_config_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Modifie une valeur spécifique dans un fichier de config TOML
/// key format: "section.field" (ex: "agent.socket_path", "output.colors")
pub fn set_config_value(path: &PathBuf, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Lire le fichier TOML existant ou créer un nouveau
    let content = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };
    
    // Parser en toml::Table
    let mut table: toml::Table = if content.is_empty() {
        toml::Table::new()
    } else {
        toml::from_str(&content)?
    };
    
    // Parser la clé "section.field"
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid key format '{}'. Expected 'section.field'", key).into());
    }
    
    let section = parts[0];
    let field = parts[1];
    
    // Créer la section si elle n'existe pas
    if !table.contains_key(section) {
        table.insert(section.to_string(), toml::Value::Table(toml::Table::new()));
    }
    
    // Récupérer la section (doit être une table)
    let section_value = table.get_mut(section)
        .ok_or(format!("Section '{}' not found", section))?;
    
    let section_table = section_value.as_table_mut()
        .ok_or(format!("'{}' is not a section", section))?;
    
    // Parser la valeur selon le type attendu
    let parsed_value = parse_value_for_field(section, field, value)?;
    
    // Insérer la valeur
    section_table.insert(field.to_string(), parsed_value);
    
    // Sauvegarder le fichier
    let toml_string = toml::to_string_pretty(&table)?;
    
    // Créer le répertoire parent si nécessaire
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, toml_string)?;
    Ok(())
}

/// Parse une valeur selon le contexte (section.field)
fn parse_value_for_field(section: &str, field: &str, value: &str) -> Result<toml::Value, Box<dyn std::error::Error>> {
    // Détecter le type selon le champ
    match (section, field) {
        // Booleans
        ("agent", "verbose") |
        ("output", "colors") |
        ("output", "show_timestamps") |
        ("output", "show_pid") => {
            let bool_val = value.to_lowercase() == "true";
            Ok(toml::Value::Boolean(bool_val))
        }
        
        // Integers
        ("agent", "connection_timeout") |
        ("agent", "retry_attempts") |
        ("performance", "buffer_size") |
        ("performance", "flush_interval") => {
            let int_val: i64 = value.parse()
                .map_err(|_| format!("'{}' is not a valid integer", value))?;
            Ok(toml::Value::Integer(int_val))
        }
        
        // Enums
        ("output", "format") => {
            match value.to_lowercase().as_str() {
                "colored" | "plain" | "json" => Ok(toml::Value::String(value.to_lowercase())),
                _ => Err(format!("Invalid format '{}'. Must be: colored, plain, json", value).into())
            }
        }
        
        ("filters", "min_level") => {
            match value.to_lowercase().as_str() {
                "debug" | "info" | "warn" | "error" => Ok(toml::Value::String(value.to_lowercase())),
                _ => Err(format!("Invalid log level '{}'. Must be: debug, info, warn, error", value).into())
            }
        }
        
        // Arrays (default_command, ignore_patterns)
        ("agent", "default_command") |
        ("filters", "ignore_patterns") => {
            // Parser comme JSON array ou string séparé par des virgules
            if value.starts_with('[') {
                // JSON array format
                let array: Vec<String> = serde_json::from_str(value)
                    .map_err(|_| format!("Invalid array format '{}'. Use JSON array like [\"npm\", \"start\"]", value))?;
                let toml_array: Vec<toml::Value> = array.into_iter()
                    .map(toml::Value::String)
                    .collect();
                Ok(toml::Value::Array(toml_array))
            } else {
                // Comma-separated format
                let parts: Vec<toml::Value> = value.split(',')
                    .map(|s| toml::Value::String(s.trim().to_string()))
                    .collect();
                Ok(toml::Value::Array(parts))
            }
        }
        
        // Default: strings
        _ => Ok(toml::Value::String(value.to_string()))
    }
}
