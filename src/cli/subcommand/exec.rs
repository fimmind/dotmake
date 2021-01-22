use crate::config::CONFIG;
use crate::identifier::Identifier;
use std::error::Error;
use structopt::StructOpt;

/// Perform nth action of a given rule
#[derive(Debug, StructOpt)]
pub struct Exec {
    /// A given rule
    #[structopt(required = true)]
    rule: Identifier,

    /// Index of an action to perform (counting from 1)
    #[structopt(required = true)]
    n: usize,
}

impl Exec {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        Ok(CONFIG.get_rule(&self.rule)?.perform_nth(self.n)?)
    }
}
