use crate::types::{CommandMessage, CommandResponse, LogMessage, RestartCommand};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::sync::{Mutex, mpsc};

pub const SOCKET_PATH: &str = "/tmp/log-agent.sock";

/// Client Unix socket pour envoyer les logs au serveur MCP
/// et recevoir des commandes du serveur
pub struct SocketClient {
    socket_path: String,
    stream: Arc<Mutex<Option<UnixStream>>>,
}

impl SocketClient {
    pub fn new(socket_path: Option<String>) -> Self {
        Self {
            socket_path: socket_path.unwrap_or_else(|| SOCKET_PATH.to_string()),
            stream: Arc::new(Mutex::new(None)),
        }
    }

    /// Établit une connexion persistante au serveur MCP
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let mut guard = self.stream.lock().await;
        *guard = Some(stream);
        Ok(())
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
                        eprintln!("✓ Connected to MCP server");
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
        // Essayer d'utiliser la connexion existante
        let mut guard = self.stream.lock().await;

        if guard.is_none() {
            // Pas de connexion, essayer de se connecter
            drop(guard); // Release lock before connecting
            self.connect().await?;
            guard = self.stream.lock().await;
        }

        if let Some(stream) = guard.as_mut() {
            // Sérialisation en JSON + nouvelle ligne
            let json = serde_json::to_string(log)?;
            let message = format!("{}\n", json);

            // Envoi
            match stream.write_all(message.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    // Connexion perdue, réinitialiser
                    *guard = None;
                    Err(e.into())
                }
            }
        } else {
            Err("Not connected".into())
        }
    }

    /// Envoie une réponse de commande au serveur MCP
    pub async fn send_command_response(
        &self,
        response: &CommandResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.stream.lock().await;

        if guard.is_none() {
            drop(guard);
            self.connect().await?;
            guard = self.stream.lock().await;
        }

        if let Some(stream) = guard.as_mut() {
            let json = serde_json::to_string(response)?;
            let message = format!("{}\n", json);

            match stream.write_all(message.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    *guard = None;
                    Err(e.into())
                }
            }
        } else {
            Err("Not connected".into())
        }
    }

    /// Démarre un listener pour recevoir les commandes du serveur MCP
    pub async fn start_command_listener(
        self: Arc<Self>,
        project: String,
        restart_tx: mpsc::Sender<RestartCommand>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // On ne peut pas lire et écrire en même temps sur le même stream
        // On va créer une nouvelle connexion dédiée à la lecture des commandes
        let socket_path = self.socket_path.clone();

        loop {
            // Créer une connexion dédiée pour recevoir les commandes
            match UnixStream::connect(&socket_path).await {
                Ok(mut stream) => {
                    // Send identification message to let server know this is a command listener
                    let hello_msg = format!(
                        r#"{{"version":"1.0","type":"agent_hello","data":{{"project":"{}","mode":"command_listener"}}}}"#,
                        project
                    );
                    if (stream
                        .write_all(format!("{}\n", hello_msg).as_bytes())
                        .await)
                        .is_err()
                    {
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                        continue;
                    }

                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();

                    loop {
                        line.clear();
                        match reader.read_line(&mut line).await {
                            Ok(0) => {
                                // EOF - connexion fermée
                                break;
                            }
                            Ok(_) => {
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    continue;
                                }

                                // Parser le message
                                if let Ok(msg) = serde_json::from_str::<CommandMessage>(trimmed)
                                    && msg.msg_type == "command"
                                    && msg.data.project == project
                                {
                                    // C'est une commande pour ce projet
                                    if msg.data.command == "restart" {
                                        let cmd = RestartCommand {
                                            request_id: msg.data.request_id.clone(),
                                            project: msg.data.project.clone(),
                                        };

                                        // Envoyer au channel de restart
                                        if restart_tx.send(cmd).await.is_err() {
                                            eprintln!("Failed to send restart command to handler");
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                // Erreur de lecture
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    // Connexion échouée, attendre avant de retry
                }
            }

            // Attendre un peu avant de retry
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    /// Test la connexion au socket
    pub async fn test_connection(
        &self,
        message: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_log = LogMessage::new(
            "test".to_string(),
            message.unwrap_or_else(|| "Test message from log-agent".to_string()),
            crate::types::LogSource::Stdout,
            std::process::id(),
        );

        self.send_log(&test_log).await
    }
}
