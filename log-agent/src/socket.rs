use crate::types::LogMessage;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use tokio::sync::mpsc;

pub const SOCKET_PATH: &str = "/tmp/log-agent.sock";

/// Client Unix socket pour envoyer les logs au serveur MCP
pub struct SocketClient {
    socket_path: String,
}

impl SocketClient {
    pub fn new(socket_path: Option<String>) -> Self {
        Self {
            socket_path: socket_path.unwrap_or_else(|| SOCKET_PATH.to_string()),
        }
    }

    /// Démarre le worker qui envoie les logs depuis le channel vers le socket
    pub async fn start_worker(
        &self,
        mut rx: mpsc::Receiver<LogMessage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut connection_logged = false;

        while let Some(log) = rx.recv().await {
            // Tentative de connexion et d'envoi
            match self.send_log(&log).await {
                Ok(_) => {
                    if !connection_logged {
                        println!("✓ Connected to MCP server at {}", self.socket_path);
                        connection_logged = true;
                    }
                }
                Err(e) => {
                    if connection_logged {
                        eprintln!("⚠ Lost connection to MCP server: {}", e);
                        connection_logged = false;
                    }
                    // On continue même si le socket n'est pas disponible
                }
            }
        }

        Ok(())
    }

    /// Envoie un log au serveur via Unix socket
    async fn send_log(&self, log: &LogMessage) -> Result<(), Box<dyn std::error::Error>> {
        // Tentative de connexion au socket
        let mut stream = UnixStream::connect(&self.socket_path).await?;

        // Sérialisation en JSON + nouvelle ligne
        let json = serde_json::to_string(log)?;
        let message = format!("{}\n", json);

        // Envoi
        stream.write_all(message.as_bytes()).await?;

        Ok(())
    }

    /// Test la connexion au socket
    pub async fn test_connection(
        &self,
        message: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing connection to {}...", self.socket_path);

        let test_log = LogMessage::new(
            "test".to_string(),
            message.unwrap_or_else(|| "Test message from log-agent".to_string()),
            crate::types::LogSource::Stdout,
            std::process::id(),
        );

        match self.send_log(&test_log).await {
            Ok(_) => {
                println!("✓ Successfully sent test message to MCP server");
                Ok(())
            }
            Err(e) => {
                eprintln!("✗ Failed to connect to MCP server: {}", e);
                eprintln!("  Make sure the MCP server is running (bun run mcp-logs/index.ts)");
                Err(e)
            }
        }
    }
}
