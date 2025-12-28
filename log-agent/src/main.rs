mod capture;
mod cli;
mod socket;
mod types;

use capture::ProcessCapture;
use clap::Parser;
use cli::{Cli, Commands};
use owo_colors::OwoColorize;
use socket::SocketClient;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { project, cmd } => {
            run_command(project, cmd).await?;
        }
        Commands::Test { message } => {
            test_connection(message).await?;
        }
    }

    Ok(())
}

async fn run_command(project: String, cmd: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("{}", format!("📋 Project: {}", project).bright_cyan());
    eprintln!();

    // Créer un channel pour les logs
    let (tx, rx) = mpsc::channel(1000);

    // Démarrer le worker socket dans une tâche séparée
    let socket_client = SocketClient::new(None);
    let socket_task = tokio::spawn(async move {
        if let Err(e) = socket_client.start_worker(rx).await {
            eprintln!("{}", format!("Socket worker error: {}", e).red());
        }
    });

    // Créer et lancer la capture du processus
    let capture = ProcessCapture::new(project, cmd);

    // Lancer la capture (bloquant jusqu'à ce que le processus se termine)
    let capture_handle = capture.spawn_with_tx(tx);

    // Attendre la fin du processus
    match capture_handle.await {
        Ok(Ok(_)) => {},
        Ok(Err(e)) => {
            eprintln!("{}", format!("\nProcess error: {}", e).red());
        }
        Err(e) => {
            eprintln!("{}", format!("\nTask error: {}", e).red());
        }
    }

    // Attendre que le worker socket se termine
    let _ = socket_task.await;

    Ok(())
}

async fn test_connection(message: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let socket_client = SocketClient::new(None);
    socket_client.test_connection(message).await
}
