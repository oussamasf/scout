use std::fs::File;
use std::io::{self, Write};

use crate::analyzer::LogAnalysis;

pub fn generate_report(analysis: &LogAnalysis) -> String {
    let mut report = String::new();

    report.push_str(&format!("Total entries: {}\n", analysis.total_entries));
    report.push_str(&format!("Info Count: {}\n", analysis.info_count));
    report.push_str(&format!("Debug Count: {}\n", analysis.debug_count));
    report.push_str(&format!("Error Count: {}\n", analysis.error_count));
    report.push_str(&format!("Warnings Count: {}\n", analysis.warnings_count));

    for (user, count) in &analysis.user_actions {
        report.push_str(&format!("User:{} made {} actions\n", user, count));
    }

    report
}

pub fn print_report(report: &String) {
    println!("report: {}", report)
}

pub fn save_report(report: &String, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(report.as_bytes())?;
    Ok(())
}
