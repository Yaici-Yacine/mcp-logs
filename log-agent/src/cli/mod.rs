use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcp-log-agent")]
#[command(about = "Real-time log capture CLI for development projects", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a command and capture its logs in real-time
    Run {
        /// Project name for identification (overrides config)
        #[arg(short, long)]
        project: Option<String>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Watch mode - TUI with restart capability (press 'r' to restart, 'q' to quit)
        #[arg(short, long)]
        watch: bool,

        /// Command to run (e.g., "bun dev", "cargo run", "npm start")
        /// If not provided, uses default_command from config
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        cmd: Vec<String>,
    },

    /// Test socket connection to MCP server
    Test {
        /// Send a test message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Create a new configuration file
    Init {
        /// Create global configuration
        #[arg(short, long)]
        global: bool,
        
        /// Create local configuration
        #[arg(short, long)]
        local: bool,
    },
    
    /// Show active configuration
    Show {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., "agent.socket_path")
        key: String,
    },
    
    /// Set a configuration value
    Set {
        /// Target global configuration
        #[arg(short, long)]
        global: bool,
        
        /// Configuration key
        key: String,
        
        /// Configuration value
        value: String,
    },
    
    /// List all configuration keys
    List,
    
    /// Validate configuration files
    Validate,
    
    /// Detect configuration sources
    Detect,
    
    /// Reset configuration to defaults
    Reset {
        /// Reset global configuration
        #[arg(short, long)]
        global: bool,
        
        /// Reset local configuration
        #[arg(short, long)]
        local: bool,
    },
    
    /// Manage color schemes
    Colors {
        #[command(subcommand)]
        action: ColorAction,
    },
}

#[derive(Subcommand)]
pub enum ColorAction {
    /// List available color schemes
    List,
    
    /// Set a color scheme
    Set {
        /// Scheme name
        scheme: String,
    },
    
    /// Preview a color scheme
    Preview {
        /// Scheme name
        scheme: String,
    },
    
    /// Test colors with example output
    Test,
}
