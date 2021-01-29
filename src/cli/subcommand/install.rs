use crate::config::CONFIG;
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
        let graph = CONFIG.get_deps_graph()?;
        let resolved = graph.resolve(self.rules.iter().collect())?;
        for ident in resolved {
            print_info!("Performing `{}`...", ident);
            CONFIG.try_get_rule(&ident)?.perform()?;
        }
        Ok(())
    }
}
