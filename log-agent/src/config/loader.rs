use super::types::Config;
use super::themes::{ThemeManager, ThemeConfig};
use std::fs;
use std::path::PathBuf;

/// Charge la configuration depuis toutes les sources et fusionne
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut merged_table = toml::Table::new();

    // 1. Charger config globale si elle existe et merger
    let global_config_dir = get_global_config_dir();
    
    if let Some(global_path) = get_global_config_path()
        && global_path.exists() {
            let contents = fs::read_to_string(&global_path)?;
            let global_table: toml::Table = toml::from_str(&contents)?;
            merge_toml_tables(&mut merged_table, global_table);
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

    // 5. Charger le thème et appliquer les couleurs
    if let Some(config_dir) = global_config_dir {
        let theme_manager = ThemeManager::new(config_dir.clone());
        
        // Initialiser les thèmes par défaut si nécessaire
        let _ = theme_manager.initialize_default_themes();
        
        // Charger le thème spécifié (ou "default" si erreur)
        let theme = theme_manager.load_theme(&config.theme)
            .unwrap_or_else(|_| {
                eprintln!("Warning: Could not load theme '{}', using default", config.theme);
                theme_manager.load_theme("default")
                    .unwrap_or_else(|_| create_fallback_theme())
            });
        
        // Appliquer les couleurs du thème
        config.colors = theme.colors;
        config.performance.tui.colors = theme.tui;
    } else {
        // Pas de config dir, utiliser le thème par défaut
        let theme = create_fallback_theme();
        config.colors = theme.colors;
        config.performance.tui.colors = theme.tui;
    }

    Ok(config)
}

/// Charge la configuration depuis un fichier spécifique (sans fusion ni thème)
pub fn load_config_from_file(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

/// Crée un thème de fallback en cas d'erreur
fn create_fallback_theme() -> ThemeConfig {
    super::themes::ThemeConfig {
        name: "fallback".to_string(),
        description: None,
        author: None,
        colors: super::types::ColorConfig::default(),
        tui: super::types::TuiColorConfig::default(),
    }
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
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_CONNECTION_TIMEOUT")
        && let Ok(timeout) = val.parse() {
            config.agent.connection_timeout = timeout;
        }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_RETRY_ATTEMPTS")
        && let Ok(attempts) = val.parse() {
            config.agent.retry_attempts = attempts;
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
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_BUFFER_SIZE")
        && let Ok(size) = val.parse() {
            config.performance.buffer_size = size;
        }
    if let Ok(val) = std::env::var("MCP_LOG_AGENT_FLUSH_INTERVAL")
        && let Ok(interval) = val.parse() {
            config.performance.flush_interval = interval;
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
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_ERROR_FG")
        && let Some(color) = parse_color(&val) {
            config.colors.error.fg = Some(color);
        }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_WARN_FG")
        && let Some(color) = parse_color(&val) {
            config.colors.warn.fg = Some(color);
        }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_INFO_FG")
        && let Some(color) = parse_color(&val) {
            config.colors.info.fg = Some(color);
        }
    if let Ok(val) = std::env::var("MCP_LOG_COLOR_DEBUG_FG")
        && let Some(color) = parse_color(&val) {
            config.colors.debug.fg = Some(color);
        }

    config
}

/// Parse une couleur depuis une string
fn parse_color(s: &str) -> Option<super::types::Color> {
    use super::types::{Color, ColorName};
    
    // Check if it's a hex color
    if s.starts_with('#') || (s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit())) {
        return Some(Color::Hex(if s.starts_with('#') { s.to_string() } else { format!("#{}", s) }));
    }
    
    // Check if it's RGB format (r,g,b)
    if s.contains(',') {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 3
            && let (Ok(r), Ok(g), Ok(b)) = (parts[0].trim().parse(), parts[1].trim().parse(), parts[2].trim().parse()) {
                return Some(Color::Rgb(r, g, b));
            }
    }
    
    // Named colors
    match s.to_lowercase().as_str() {
        "black" => Some(Color::Named(ColorName::Black)),
        "red" => Some(Color::Named(ColorName::Red)),
        "green" => Some(Color::Named(ColorName::Green)),
        "yellow" => Some(Color::Named(ColorName::Yellow)),
        "blue" => Some(Color::Named(ColorName::Blue)),
        "magenta" => Some(Color::Named(ColorName::Magenta)),
        "cyan" => Some(Color::Named(ColorName::Cyan)),
        "white" => Some(Color::Named(ColorName::White)),
        "bright_black" => Some(Color::Named(ColorName::BrightBlack)),
        "bright_red" => Some(Color::Named(ColorName::BrightRed)),
        "bright_green" => Some(Color::Named(ColorName::Green)),
        "bright_yellow" => Some(Color::Named(ColorName::BrightYellow)),
        "bright_blue" => Some(Color::Named(ColorName::BrightBlue)),
        "bright_magenta" => Some(Color::Named(ColorName::BrightMagenta)),
        "bright_cyan" => Some(Color::Named(ColorName::BrightCyan)),
        "bright_white" => Some(Color::Named(ColorName::BrightWhite)),
        _ => None,
    }
}

/// Retourne le chemin du dossier de config globale
pub fn get_global_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("mcp-log-agent");
        path
    })
}

/// Retourne le chemin de la config globale
pub fn get_global_config_path() -> Option<PathBuf> {
    get_global_config_dir().map(|mut path| {
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
    let commented_toml = r###"# MCP Log Agent Configuration File
# Configuration priority (highest to lowest):
#   1. CLI arguments
#   2. Environment variables (MCP_LOG_*)
#   3. Local config file (.mcp-log-agent.toml)
#   4. Global config file (~/.config/mcp-log-agent/config.toml)
#   5. Default values

# ============================================================================
# Theme Configuration
# ============================================================================
# theme: Name of the color theme to use (loaded from ~/.config/mcp-log-agent/themes/)
# Default: "default"
# Available themes: default, dracula, nord, monokai, solarized-dark, minimal
#
# To create a custom theme:
#   1. Go to ~/.config/mcp-log-agent/themes/
#   2. Copy an existing theme file (e.g., default.toml)
#   3. Modify colors to your liking
#   4. Set theme = "your-theme-name" below
#
# To list available themes:
#   mcp-log-agent config theme list
#
# Theme files control:
#   - Log level colors (error, warn, info, debug)
#   - TUI interface colors (header, status bar, borders, etc.)
#   - System message colors

theme = "default"

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
# Uncomment and modify to use:
# default_command = ["npm", "start"]

# watch: Enable TUI (Terminal User Interface) watch mode by default
# Default: false
# When true, all runs use interactive TUI mode unless overridden with CLI
# Press '?' in TUI mode to see all available shortcuts
# Env var: MCP_LOG_AGENT_WATCH
watch = false

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

# auto_quit: Automatically quit TUI when the monitored process exits
# Default: false
# When true, TUI closes automatically after process terminates
# When false, TUI stays open to review logs after process ends
# Env var: MCP_LOG_AGENT_AUTO_QUIT
auto_quit = false

# ============================================================================
# [agent.commands] - Predefined commands for quick access
# ============================================================================
# Define multiple commands that you can run with: mcp-log-agent run --cmd <name>
# This is useful for projects with multiple dev/build/test commands
#
# Usage examples:
#   mcp-log-agent run --cmd dev         # Runs the "dev" command
#   mcp-log-agent run --cmd build       # Runs the "build" command
#   mcp-log-agent run -w --cmd test     # Runs "test" in TUI mode
#
# Uncomment and customize examples below:

[agent.commands]
# dev = ["npm", "run", "dev"]
# build = ["npm", "run", "build"]
# test = ["npm", "test"]
# start = ["npm", "start"]

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

# ============================================================================
# [performance.tui] - TUI (Terminal User Interface) performance settings
# ============================================================================
[performance.tui]

# max_logs: Maximum number of logs kept in TUI memory
# Default: 5000
# Older logs are automatically discarded when this limit is reached
max_logs = 5000

# tick_rate_ms: Tick rate for countdown timer in milliseconds
# Default: 250 (4 updates per second)
# Controls how often the countdown and stats update
tick_rate_ms = 250

# frame_rate_ms: Frame rate limit in milliseconds
# Default: 100 (10 FPS)
# Prevents lag with high-frequency log output
frame_rate_ms = 100
"###;
    
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

/// Vérifie si le dossier courant est un repo Git
pub fn is_git_repository() -> bool {
    PathBuf::from(".git").exists()
}

/// Vérifie si le fichier de config est déjà dans .gitignore
pub fn is_config_in_gitignore(config_filename: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let gitignore_path = PathBuf::from(".gitignore");
    
    if !gitignore_path.exists() {
        return Ok(false);
    }
    
    let content = fs::read_to_string(&gitignore_path)?;
    
    // Vérifier si le fichier est déjà mentionné (ligne exacte ou pattern)
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == config_filename || trimmed == format!("/{}", config_filename) {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// Ajoute le fichier de config au .gitignore
pub fn add_to_gitignore(config_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let gitignore_path = PathBuf::from(".gitignore");
    
    let mut content = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };
    
    // Ajouter une nouvelle ligne si le fichier ne se termine pas par un newline
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    
    // Ajouter un commentaire et le fichier
    if !content.contains("# mcp-log-agent") {
        content.push_str("\n# mcp-log-agent local configuration\n");
    }
    content.push_str(config_filename);
    content.push('\n');
    
    fs::write(&gitignore_path, content)?;
    
    Ok(())
}

/// Demande à l'utilisateur s'il veut ajouter le fichier au .gitignore
pub fn prompt_add_to_gitignore(config_filename: &str) -> bool {
    use std::io::{self, Write};
    
    print!("\n📝 Add {} to .gitignore? [Y/n]: ", config_filename);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().to_lowercase();
    // Par défaut "yes" si l'utilisateur appuie juste sur Enter
    input.is_empty() || input == "y" || input == "yes"
}
