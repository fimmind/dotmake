use structopt::StructOpt;
use std::error::Error;
use super::Options;

#[derive(Debug, StructOpt)]
pub enum Subcommand {}

impl Subcommand {
    pub fn perform(&self, options: &Options) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
