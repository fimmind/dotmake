mod install;

use structopt::StructOpt;
use std::error::Error;

use install::Install;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Install(Install),
}

impl Subcommand {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::Install(sub) => sub.perform(),
        }
    }
}
