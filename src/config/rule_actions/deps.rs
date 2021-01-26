use super::{Action, RuleActionsConf};
use crate::config::deserializers::deserialize_identifiers_set;
use crate::identifier::Identifier;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Deps {
    #[serde(deserialize_with = "deserialize_identifiers_set")]
    deps: HashSet<Identifier>,
}

impl Action for Deps {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_deps(&self) -> HashSet<Identifier> {
        self.deps.clone()
    }
}
