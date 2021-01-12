use crate::config::Identifier;
use std::error::Error;
use structopt::StructOpt;

use crate::config::CONFIG;

use crate::deps_resolver::DepsResolver;

/// Perform installation of given rules
#[derive(Debug, StructOpt)]
pub struct Install {
    /// Rules to be installed
    #[structopt(required = true)]
    rules: Vec<Identifier>,
}

impl Install {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        let resolver = DepsResolver::init(self.rules.clone(), |ident| CONFIG.get_rule_deps_conf(ident));
        for ident in resolver.try_resolve()? {
            CONFIG.perform_rule(ident)?;
        }
        Ok(())
    }
}
