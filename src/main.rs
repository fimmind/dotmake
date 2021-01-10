mod cli;

fn main() {
    cli::SUBCOMMAND
        .perform()
        .unwrap_or_else(|_| todo!("Error handling"))
}
