mod cli;

fn main() {
    let matches = cli::run_cli().get_matches();
    cli::handle_matches(&matches);
}
