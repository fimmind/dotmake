mod deps_graph;

use crate::deserializers::identifiers_set;
use crate::identifier::Identifier;
use deps_graph::{DepsGraph, DepsIter};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DepsConf {
    #[serde(default, deserialize_with = "identifiers_set")]
    pub deps: HashSet<Identifier>,

    #[serde(default, deserialize_with = "identifiers_set")]
    pub post: HashSet<Identifier>,
}

pub struct DepsResolver<'a> {
    deps_graph: DepsGraph<&'a Identifier>,
}

impl<'a> DepsResolver<'a> {
    pub fn init<F>(roots: impl IntoIterator<Item = &'a Identifier>, get_deps_for: F) -> Self
    where
        F: Fn(&'a Identifier) -> &'a DepsConf,
    {
        DepsResolver {
            deps_graph: Self::build_deps_graph(get_deps_for),
        }
    }

    fn build_deps_graph<F>(get_deps_for: F) -> DepsGraph<&'a Identifier>
    where
        F: Fn(&'a Identifier) -> &'a DepsConf,
    {
        todo!("Dependecies resolving")
    }

    pub fn try_resolve(self) -> Result<DepsIter<&'a Identifier>, Box<dyn Error>> {
        todo!("try_resolve_deps")
    }
}
