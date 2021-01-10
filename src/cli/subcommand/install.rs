use structopt::StructOpt;
use std::error::Error;

/// Perform installation of given rules
#[derive(Debug, StructOpt)]
pub struct Install {
    /// Rules to be installed
    #[structopt(required = true)]
    rules: Vec<String>,
}

impl Install {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        todo!("Install subcommand")
    }
}
