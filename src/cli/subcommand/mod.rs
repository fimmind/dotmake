//! StructOpt's subcommand scructure

mod add;
mod completion;
mod exec;
mod install;

use std::error::Error;
use structopt::StructOpt;

use add::Add;
use completion::Completion;
use exec::Exec;
use install::Install;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Install(Install),
    Exec(Exec),
    Completion(Completion),
    Add(Add),
}

impl Subcommand {
    /// Run the subcommand
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::Install(sub) => sub.perform(),
            Subcommand::Exec(sub) => sub.perform(),
            Subcommand::Completion(sub) => sub.perform(),
            Subcommand::Add(sub) => sub.perform(),
        }
    }
}
