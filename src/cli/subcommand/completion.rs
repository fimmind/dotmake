use std::env;
use std::io;
use structopt::clap::Shell;
use structopt::StructOpt;

/// Generate a completion script for a given shell
#[derive(Debug, StructOpt)]
pub struct Completion {
    #[structopt(default_value = "bash")]
    shell: Shell,
}

impl Completion {
    pub fn perform(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(crate::cli::Cli::clap().gen_completions_to(
            env!("CARGO_BIN_NAME"),
            self.shell,
            &mut io::stdout(),
        ))
    }
}