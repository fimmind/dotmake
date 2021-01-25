use crate::config::CONFIG;
use crate::deps_graph::DepsGraph;
use crate::identifier::Identifier;
use std::error::Error;
use structopt::StructOpt;

/// Perform installation of given rules
#[derive(Debug, StructOpt)]
pub struct Install {
    /// Rules to be installed
    #[structopt(required = true)]
    rules: Vec<Identifier>,
}

impl Install {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        let resolver = DepsGraph::build(&self.rules, |ident| {
            CONFIG
                .get_rule(ident)
                .unwrap_or_else(exit_error_fn!())
                .deps_conf()
        });
        let resolved = resolver
            .resolve()
            .map_err(|cycle| format!("Found cycle in dpendencies: {}", cycle))?;
        for ident in resolved {
            print_info!("Performing `{}`...", ident);
            CONFIG.get_rule(ident)?.perform()?;
        }
        Ok(())
    }
}
