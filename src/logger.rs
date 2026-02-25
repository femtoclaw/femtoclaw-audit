//! Structured Logging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub target: String,
    pub fields: serde_json::Value,
}

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self
    }

    pub fn log(&self, level: Level, target: &str, message: &str, fields: serde_json::Value) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: level.to_string(),
            message: message.to_string(),
            target: target.to_string(),
            fields,
        };
        tracing::debug!(?entry);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
