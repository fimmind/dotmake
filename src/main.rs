#[macro_use]
mod io;
mod cli;

fn main() {
    cli::SUBCOMMAND.perform().unwrap_or_else(exit_error_fn!())
}
