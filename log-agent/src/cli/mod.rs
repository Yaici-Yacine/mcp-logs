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

        /// Use a predefined command from config (e.g., --cmd dev, --cmd test)
        /// Looks up the command in [agent.commands] section of config
        #[arg(short = 'C', long)]
        cmd: Option<String>,

        /// Command to run (e.g., "bun dev", "cargo run", "npm start")
        /// If not provided, uses --cmd or default_command from config
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
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
        
        /// Skip .gitignore prompt (don't add to .gitignore)
        #[arg(short = 'n', long)]
        no_gitignore: bool,
        
        /// Automatically add to .gitignore without prompting
        #[arg(short = 'y', long)]
        yes: bool,
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
    
    /// Manage themes
    Theme {
        #[command(subcommand)]
        action: ThemeAction,
    },
}

#[derive(Subcommand)]
pub enum ThemeAction {
    /// List available themes
    List,
    
    /// Show theme details
    Show {
        /// Theme name
        name: String,
    },
    
    /// Create a new theme
    Create {
        /// Theme name
        name: String,
        
        /// Copy from existing theme
        #[arg(short, long)]
        from: Option<String>,
        
        /// Interactive mode (prompt for colors)
        #[arg(short, long)]
        interactive: bool,
    },
    
    /// Export current colors as a theme
    Export {
        /// Theme name
        name: String,
        
        /// Theme description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Theme author
        #[arg(short, long)]
        author: Option<String>,
    },
    
    /// Set active theme
    Set {
        /// Theme name
        name: String,
        
        /// Set globally
        #[arg(short, long)]
        global: bool,
    },
    
    /// Preview a theme with example output
    Preview {
        /// Theme name
        name: String,
    },
}
