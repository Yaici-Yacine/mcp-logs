use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "log-agent")]
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
        /// Project name for identification
        #[arg(short, long, default_value = "default")]
        project: String,

        /// Command to run (e.g., "bun dev", "cargo run", "npm start")
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        cmd: Vec<String>,
    },

    /// Test socket connection to MCP server
    Test {
        /// Send a test message
        #[arg(short, long)]
        message: Option<String>,
    },
}
