use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub context: serde_json::Value,
}
