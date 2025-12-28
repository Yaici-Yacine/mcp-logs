use crate::types::{LogLevel, LogMessage, LogSource};
use owo_colors::OwoColorize;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

pub struct ProcessCapture {
    pub project: String,
    pub command: Vec<String>,
}

impl ProcessCapture {
    pub fn new(project: String, command: Vec<String>) -> Self {
        Self { project, command }
    }

    /// Lance le processus et retourne un handle
    pub fn spawn_with_tx(
        self,
        tx: mpsc::Sender<LogMessage>,
    ) -> tokio::task::JoinHandle<Result<(), String>> {
        tokio::spawn(async move {
            self.run(tx).await.map_err(|e| e.to_string())
        })
    }

    /// Lance le processus et capture les logs stdout/stderr
    async fn run(
        self,
        tx: mpsc::Sender<LogMessage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.command.is_empty() {
            return Err("No command provided".into());
        }

        let program = &self.command[0];
        let args = &self.command[1..];

        eprintln!(
            "{}",
            format!(
                "🚀 Starting '{}': {} {}",
                self.project,
                program,
                args.join(" ")
            )
            .bright_green()
        );

        // Spawn le processus enfant
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id().ok_or("Failed to get PID")?;
        eprintln!("{}", format!("✓ Process started (PID: {})", pid).bright_black());

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

        // Capture stdout
        let tx_stdout = tx.clone();
        let project_stdout = self.project.clone();
        let stdout_task = tokio::spawn(async move {
            capture_stream(
                BufReader::new(stdout),
                project_stdout,
                LogSource::Stdout,
                pid,
                tx_stdout,
            )
            .await;
        });

        // Capture stderr
        let tx_stderr = tx;
        let project_stderr = self.project.clone();
        let stderr_task = tokio::spawn(async move {
            capture_stream(
                BufReader::new(stderr),
                project_stderr,
                LogSource::Stderr,
                pid,
                tx_stderr,
            )
            .await;
        });

        // Attendre que le processus se termine
        let status = child.wait().await?;

        // Attendre que les tâches de capture se terminent
        let _ = stdout_task.await;
        let _ = stderr_task.await;

        if status.success() {
            eprintln!("{}", format!("✓ Process exited successfully").green());
        } else {
            eprintln!(
                "{}",
                format!("✗ Process exited with status: {}", status).red()
            );
        }

        Ok(())
    }
}

/// Capture un stream (stdout ou stderr) ligne par ligne
async fn capture_stream<R>(
    mut reader: BufReader<R>,
    project: String,
    source: LogSource,
    pid: u32,
    tx: mpsc::Sender<LogMessage>,
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
                    // Crée le message de log
                    let log = LogMessage::new(project.clone(), message.clone(), source.clone(), pid);
                    
                    // Affiche dans le terminal avec coloration
                    print_colored_log(&log);

                    // Envoie le log au channel
                    if let Err(e) = tx.send(log).await {
                        eprintln!("{}", format!("Failed to send log to channel: {}", e).red());
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                break;
            }
        }
    }
}

/// Affiche un log avec coloration selon le niveau
fn print_colored_log(log: &LogMessage) {
    use owo_colors::OwoColorize;
    
    // Pas de préfixe, juste le message colorisé selon le niveau
    let message_str = &log.data.message;
    
    match log.data.level {
        LogLevel::Error => {
            eprintln!("{}", message_str.red().bold());
        }
        LogLevel::Warn => {
            eprintln!("{}", message_str.yellow());
        }
        LogLevel::Debug => {
            eprintln!("{}", message_str.blue());
        }
        LogLevel::Info => {
            eprintln!("{}", message_str);
        }
    }
}
