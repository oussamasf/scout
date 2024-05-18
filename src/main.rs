use scout::cli::{handle_matches, run_cli};
fn main() {
    let matches = run_cli().get_matches();
    handle_matches(&matches);
}
