use crate::analyzer::analyze_logs;
use crate::parser::json::parse_json_log;
use crate::reporter::{generate_report, print_report, save_report};
use clap::{Arg, Command};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn deserialize(data: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(data)?;

    Ok(v)
}

pub fn run_cli() -> Command {
    Command::new("Scout")
        .about("Log analysis tool")
        .subcommand_required(true)
        .subcommand(
            Command::new("analyze")
                .about("Analyze log files")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .value_name("FILE")
                        .help("Specifies the log file to analyze")
                        .required(true),
                )
                .arg(
                    Arg::new("type")
                        .long("type")
                        .short('t')
                        .value_name("TYPE")
                        .help("Specifies the type of log file")
                        .value_parser(|s: &str| match s.to_lowercase().as_str() {
                            "json" => Ok("json".to_string()),
                            "clf" => Ok("clf".to_string()),
                            _ => Err(String::from("Invalid log type. Must be 'json' or 'clf'")),
                        }),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .value_name("OUTPUT")
                        .help("Specifies the output file for the report"),
                ),
        )
        .subcommand(
            Command::new("view")
                .about("View the contents of a log file")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .value_name("FILE")
                        .help("Specifies the log file to view")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("filter")
                .about("Filter log entries based on a query")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .value_name("FILE")
                        .help("Specifies the log file to filter")
                        .required(true),
                )
                .arg(
                    Arg::new("query")
                        .long("query")
                        .short('q')
                        .value_name("QUERY")
                        .help("Defines the query for filtering entries")
                        .required(true),
                ),
        )
        .subcommand(Command::new("version").about("Shows version information"))
}

pub fn handle_matches(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("analyze", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();

            let default_log_type = "json".to_string();
            let log_type = sub_matches
                .get_one::<String>("type")
                .unwrap_or(&default_log_type);

            let default_output = "report.txt".to_string();
            let output = sub_matches
                .get_one::<String>("output")
                .unwrap_or(&default_output);

            println!(
                "Analyzing {} as {} type. Results will be saved to {}",
                file_path, log_type, output
            );

            if log_type == "json" {
                match parse_json_log(file_path) {
                    Ok(el) => {
                        let analysis = analyze_logs(&el);
                        println!("debug_count: {}", analysis.debug_count);
                        println!("error_count: {}", analysis.error_count);
                        println!("info_count: {}", analysis.info_count);
                        println!("total_entries: {}", analysis.total_entries);
                        println!("user_actions: {:?}", analysis.user_actions);
                        println!("warnings_count: {}", analysis.warnings_count);

                        let report = generate_report(&analysis);
                        print_report(&report);
                        let _ = save_report(&report, &default_output);
                    }
                    Err(_) => println!("something went wrong"),
                }
            }
        }
        Some(("view", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();

            if let Ok(file) = File::open(file_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            } else {
                println!("Failed to open file: {}", file_path);
            }
        }
        Some(("filter", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();
            let query = sub_matches.get_one::<String>("query").unwrap();

            println!(
                "Filtering entries from {} with query '{}'",
                file_path, query
            );
            // TODO
        }
        Some(("help", _)) => {
            run_cli().print_help().unwrap();
            println!();
        }
        Some(("version", _)) => {
            println!("Scout version 1.0");
        }
        _ => unreachable!("Exhaustive subcommand matching in Clap"),
    }
}
