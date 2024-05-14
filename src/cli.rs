use clap::{arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn run_cli() -> Command<'static> {
    Command::new("Scout")
        .about("Log analysis tool")
        .subcommand_required(true)
        .subcommand(
            Command::new("analyze")
                .about("Analyze log files")
                .arg(arg!(-f --file <FILE> "Specifies the log file to analyze").required(true))
                .arg(arg!(-t --type <TYPE> "Specifies the type of log file"))
                .arg(arg!(-o --output <OUTPUT> "Specifies the output file for the report")),
        )
        .subcommand(
            Command::new("view")
                .about("View the contents of a log file")
                .arg(arg!(-f --file <FILE> "Specifies the log file to view").required(true)),
        )
        .subcommand(
            Command::new("filter")
                .about("Filter log entries based on a query")
                .arg(arg!(-f --file <FILE> "Specifies the log file to filter").required(true))
                .arg(
                    arg!(-q --query <QUERY> "Defines the query for filtering entries")
                        .required(true),
                ),
        )
        .subcommand(Command::new("help").about("Shows help information"))
        .subcommand(Command::new("version").about("Shows version information"));
}

pub fn handle_matches(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("analyze", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();
            let log_type = sub_matches
                .get_one::<String>("type")
                .unwrap_or(&"default".to_string());
            let output = sub_matches
                .get_one::<String>("output")
                .unwrap_or(&"report.txt".to_string());

            println!(
                "Analyzing {} as {} type. Results will be saved to {}",
                file_path, log_type, output
            );
            // TODO
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
            app.print_help().unwrap();
            println!();
        }
        Some(("version", _)) => {
            println!("Scout version 1.0");
        }
        _ => unreachable!("Exhaustive subcommand matching in Clap"),
    }
}
