use super::{Action, RuleActionsConf};
use crate::identifier::{Identifier, Identifiers};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Deps {
    deps: Identifiers,
}

impl Action for Deps {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.deps.into_iter().collect()
    }
}
