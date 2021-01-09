mod cli;
use cli::Cli;

fn main() {
    Cli::init().run_subcommand().unwrap()
}
