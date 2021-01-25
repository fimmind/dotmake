use super::{Action, RuleActionsConf};
use crate::config::deserializers::deserialize_identifiers_set;
use crate::deps_graph::DepsConf;
use crate::identifier::Identifier;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct Deps {
    deps: HashSet<Identifier>,
}

impl<'de> Deserialize<'de> for Deps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Deps {
            deps: deserialize_identifiers_set(deserializer)?,
        })
    }
}

impl Action for Deps {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_deps_conf(&self) -> DepsConf<Identifier> {
        let mut deps_conf = DepsConf::new();
        deps_conf.add_deps(self.deps.iter().map(Clone::clone));
        deps_conf
    }
}

#[derive(Debug)]
pub struct PostDeps {
    post_deps: HashSet<Identifier>,
}

impl<'de> Deserialize<'de> for PostDeps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PostDeps {
            post_deps: deserialize_identifiers_set(deserializer)?,
        })
    }
}

impl Action for PostDeps {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_deps_conf(&self) -> DepsConf<Identifier> {
        let mut deps_conf = DepsConf::new();
        deps_conf.add_post_deps(self.post_deps.iter().map(Clone::clone));
        deps_conf
    }
}
