//! An action that only specifies dependencies

use super::{Action, RuleActionsConf};
use crate::types::{Identifier, Identifiers};
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Deps {
    deps: Identifiers,
}

impl Action for Deps {
    fn get_deps(&self, _conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.deps.into_iter().collect()
    }
}
