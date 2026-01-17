use crate::config::Config;
use crate::types::{LogMessage, LogSource};
use std::process::ExitStatus;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use std::process::Stdio;

/// Superviseur de processus pour la TUI
pub struct Supervisor {
    project: String,
    command: Vec<String>,
    config: Config,
    child: Option<Child>,
    stdout_task: Option<tokio::task::JoinHandle<()>>,
    stderr_task: Option<tokio::task::JoinHandle<()>>,
}

impl Supervisor {
    pub fn new(project: String, command: Vec<String>, config: Config) -> Self {
        Self {
            project,
            command,
            config,
            child: None,
            stdout_task: None,
            stderr_task: None,
        }
    }

    /// Démarre le processus et retourne son PID
    pub async fn start(
        &mut self,
        tx: mpsc::Sender<LogMessage>,
    ) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        if self.command.is_empty() {
            return Err("No command provided".into());
        }

        let program = &self.command[0];
        let args = &self.command[1..];

        // Spawn le processus
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id().ok_or("Failed to get PID")?;

        // Prendre stdout et stderr
        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

        // Spawn les tâches de capture
        let project_stdout = self.project.clone();
        let config_stdout = self.config.clone();
        let tx_stdout = tx.clone();
        self.stdout_task = Some(tokio::spawn(async move {
            capture_stream(
                BufReader::new(stdout),
                project_stdout,
                LogSource::Stdout,
                pid,
                tx_stdout,
                config_stdout,
            )
            .await;
        }));

        let project_stderr = self.project.clone();
        let config_stderr = self.config.clone();
        let tx_stderr = tx;
        self.stderr_task = Some(tokio::spawn(async move {
            capture_stream(
                BufReader::new(stderr),
                project_stderr,
                LogSource::Stderr,
                pid,
                tx_stderr,
                config_stderr,
            )
            .await;
        }));

        self.child = Some(child);

        Ok(pid)
    }

    /// Arrête le processus
    pub async fn stop(&mut self) {
        if let Some(ref mut child) = self.child {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        
        // Attendre que les tâches de capture se terminent
        if let Some(task) = self.stdout_task.take() {
            let _ = task.await;
        }
        if let Some(task) = self.stderr_task.take() {
            let _ = task.await;
        }
        
        self.child = None;
    }

    /// Redémarre le processus
    pub async fn restart(
        &mut self,
        tx: mpsc::Sender<LogMessage>,
    ) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        self.stop().await;
        // Petit délai pour s'assurer que tout est clean
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        self.start(tx).await
    }

    /// Vérifie si le processus est terminé (non-bloquant)
    pub fn try_wait(&mut self) -> Option<ExitStatus> {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    // Processus terminé, nettoyer l'état
                    self.child = None;
                    
                    // Les tâches stdout/stderr vont se terminer naturellement à EOF
                    // On les garde pour finir de capturer les derniers logs
                    // Elles seront nettoyées au prochain restart() ou stop()
                    
                    Some(status)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Vérifie si le processus est en cours d'exécution
    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }
}

/// Capture un stream ligne par ligne et envoie les logs
async fn capture_stream<R>(
    mut reader: BufReader<R>,
    project: String,
    source: LogSource,
    pid: u32,
    tx: mpsc::Sender<LogMessage>,
    _config: Config,
) where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = line.trim_end().to_string();
                if !message.is_empty() {
                    let log = LogMessage::new(project.clone(), message, source.clone(), pid);
                    if tx.send(log).await.is_err() {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
}
