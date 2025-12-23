use crate::types::{LogMessage, LogSource};
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

    /// Lance le processus et capture les logs stdout/stderr
    pub async fn run(
        &self,
        tx: mpsc::Sender<LogMessage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.command.is_empty() {
            return Err("No command provided".into());
        }

        let program = &self.command[0];
        let args = &self.command[1..];

        println!(
            "🚀 Starting project '{}' with command: {} {}",
            self.project,
            program,
            args.join(" ")
        );

        // Spawn le processus enfant
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id().ok_or("Failed to get PID")?;
        println!("✓ Process started with PID: {}", pid);

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

        println!("✓ Process exited with status: {}", status);

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
                    // Affiche dans le terminal
                    match source {
                        LogSource::Stdout => println!("[stdout] {}", message),
                        LogSource::Stderr => eprintln!("[stderr] {}", message),
                    }

                    // Crée et envoie le message de log
                    let log =
                        LogMessage::new(project.clone(), message, source.clone(), pid);

                    if let Err(e) = tx.send(log).await {
                        eprintln!("Failed to send log to channel: {}", e);
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
