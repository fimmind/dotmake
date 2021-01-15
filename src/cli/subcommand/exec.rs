use structopt::StructOpt;
use std::error::Error;

use crate::config::CONFIG;
use crate::identifier::Identifier;

/// Perform specified actions for a given rule
#[derive(Debug, StructOpt)]
pub struct Exec {
    /// A given rule
    #[structopt(required = true)]
    rule: Identifier,

    /// Actions to perform
    #[structopt(required = true)]
    actions: Vec<Identifier>,
}

impl Exec {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        CONFIG.perform_rule_actions(&self.rule, &self.actions)
    }
}
