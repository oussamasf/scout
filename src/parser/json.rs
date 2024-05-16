use crate::parser::LogEntry;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

pub fn parse_json_log(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let log_entries: Vec<LogEntry> = from_reader(reader)?;
    Ok(log_entries)
}
