mod install;
mod exec;

use structopt::StructOpt;
use std::error::Error;

use install::Install;
use exec::Exec;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Install(Install),
    Exec(Exec),
}

impl Subcommand {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::Install(sub) => sub.perform(),
            Subcommand::Exec(sub) => sub.perform(),
        }
    }
}
