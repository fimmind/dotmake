use crate::identifier::Identifier;
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
        let resolver = DepsResolver::init(&self.rules, |ident| CONFIG.try_get_rule_deps_conf(ident).unwrap_or_else(exit_error_fn!()));
        for ident in resolver.try_resolve()? {
            CONFIG.perform_rule(ident)?;
        }
        Ok(())
    }
}
