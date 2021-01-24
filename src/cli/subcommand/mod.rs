mod install;
mod exec;
mod completion;

use structopt::StructOpt;
use std::error::Error;

use install::Install;
use exec::Exec;
use completion::Completion;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Install(Install),
    Exec(Exec),
    Completion(Completion),
}

impl Subcommand {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::Install(sub) => sub.perform(),
            Subcommand::Exec(sub) => sub.perform(),
            Subcommand::Completion(sub) => sub.perform(),
        }
    }
}
