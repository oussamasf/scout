# Scout

Scout is a command-line application written in Rust for analyzing, viewing, and filtering log files. It supports JSON formatted log files and provides features to analyze log data, view log contents, and filter log entries based on a query.

## Features

- **Analyze**: Compute statistics, identify errors, and extract key metrics from log files.
- **View**: Display the contents of a log file.
- **Filter**: Filter log entries based on a query string.

## Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/oussamasf/scout
   cd scout
   ```
2. **Build the project:**:
   ```sh
   cargo build --release
   ```
3. **Run the application::**:

   ```sh
   cargo run -- <COMMAND> [OPTIONS]
   ```

## Usage

### Analyze Log Files

Analyze a JSON log file and generate a report.

```sh
scout analyze --file <FILE_PATH> [--type <TYPE>] [--output <OUTPUT_FILE>]
```

- --file or -f: Specifies the log file to analyze (required).
- --type or -t: Specifies the type of log file (default: json).
- --output or -o: Specifies the output file for the report (default: report.txt).

### View Log File

View the contents of a log file.

```sh
scout view --file <FILE_PATH>
```

- --file or -f: Specifies the log file to view (required).

### Filter Log Entries

View the contents of a log file.

```sh
scout view --file <FILE_PATH>
```

- --file or -f: Specifies the log file to filter (required).
- --query or -q: Defines the query for filtering entries (required).

## Examples

### Analyze a JSON Log File

```sh
scout analyze --file src/server_logs.json --output report.txt
```

## Project Structure

```sh
scout/
├── Cargo.toml
└── src/
    ├── main.rs        # Entry point of the application
    │   lib.rs         # Library crate
    ├── cli.rs         # Handles command-line interface logic
    ├── parser/        # Directory for log parsing functionalities
    │   ├── mod.rs     # Declares the parser module and sub-modules
    │   └── json.rs    # Specific parser for JSON logs
    ├── analyzer.rs    # Analyzes parsed data
    ├──  reporter.rs    # Handles generating and outputting reports
    └── server_logs.json  # Example log file
```
