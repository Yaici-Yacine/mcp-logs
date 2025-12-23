use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Message de log envoyé au serveur MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub version: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: LogData,
}

/// Données du log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogData {
    pub timestamp: String,
    pub level: LogLevel,
    pub source: LogSource,
    pub project: String,
    pub message: String,
    pub pid: u32,
}

/// Niveau de log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// Source du log (stdout ou stderr)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogSource {
    Stdout,
    Stderr,
}

impl LogMessage {
    /// Crée un nouveau message de log
    pub fn new(project: String, message: String, source: LogSource, pid: u32) -> Self {
        let level = Self::infer_level(&message);

        Self {
            version: "1.0".to_string(),
            msg_type: "log_entry".to_string(),
            data: LogData {
                timestamp: Utc::now().to_rfc3339(),
                level,
                source,
                project,
                message,
                pid,
            },
        }
    }

    /// Infère le niveau de log depuis le message
    fn infer_level(message: &str) -> LogLevel {
        let lower = message.to_lowercase();
        if lower.contains("error") || lower.contains("err") || lower.contains("fatal") {
            LogLevel::Error
        } else if lower.contains("warn") || lower.contains("warning") {
            LogLevel::Warn
        } else if lower.contains("debug") || lower.contains("trace") {
            LogLevel::Debug
        } else {
            LogLevel::Info
        }
    }
}
