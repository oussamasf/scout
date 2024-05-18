use crate::parser::LogEntry;
use std::collections::HashMap;

pub struct LogAnalysis {
    pub total_entries: usize,
    pub error_count: usize,
    pub warnings_count: usize,
    pub info_count: usize,
    pub debug_count: usize,
    pub user_actions: HashMap<String, usize>,
}

pub fn analyze_logs(log_entries: &[LogEntry]) -> LogAnalysis {
    let mut total_entries = 0;
    let mut error_count = 0;
    let mut warnings_count = 0;
    let mut info_count = 0;
    let mut debug_count = 0;
    let mut user_actions = HashMap::new();

    for entry in log_entries {
        total_entries += 1;

        match entry.level.as_str() {
            "ERROR" => error_count += 1,
            "WARN" => warnings_count += 1,
            "INFO" => info_count += 1,
            "DEBUG" => debug_count += 1,
            _ => (),
        }

        if let Some(user_id) = entry.context.get("user_id").and_then(|v| v.as_str()) {
            *user_actions.entry(user_id.to_string()).or_insert(0) += 1;
        }
    }

    LogAnalysis {
        total_entries,
        error_count,
        warnings_count,
        info_count,
        debug_count,
        user_actions,
    }
}
