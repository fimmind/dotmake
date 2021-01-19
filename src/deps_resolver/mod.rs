mod deps_graph;

use crate::deserializers::identifiers_set;
use crate::identifier::Identifier;
use deps_graph::DepsGraph;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DepsConf {
    #[serde(default, deserialize_with = "identifiers_set")]
    deps: HashSet<Identifier>,

    #[serde(default, rename = "post", deserialize_with = "identifiers_set")]
    post_deps: HashSet<Identifier>,
}

impl DepsConf {
    pub fn deps(&self) -> impl IntoIterator<Item = &Identifier> {
        &self.deps
    }

    pub fn post_deps(&self) -> impl IntoIterator<Item = &Identifier> {
        &self.post_deps
    }

    pub fn add_dep(&mut self, dep: ) {

    }
}

pub struct DepsResolver<'a> {
    deps_graph: DepsGraph<&'a Identifier>,
}

impl<'a> DepsResolver<'a> {
    pub fn init(
        roots: impl IntoIterator<Item = &'a Identifier>,
        get_deps_for: impl Fn(&'a Identifier) -> &'a DepsConf,
    ) -> Self {
        DepsResolver {
            deps_graph: Self::build_deps_graph(roots, get_deps_for),
        }
    }

    fn build_deps_graph(
        roots: impl IntoIterator<Item = &'a Identifier>,
        get_deps_for: impl Fn(&'a Identifier) -> &'a DepsConf,
    ) -> DepsGraph<&'a Identifier> {
        let mut visited = HashSet::new();
        let mut stack = roots.into_iter().collect::<Vec<_>>();
        let mut deps_graph = DepsGraph::init(stack.iter().map(|&i| i));
        while let Some(node) = stack.pop() {
            let deps_conf = get_deps_for(node);
            for dep in deps_conf.deps() {
                deps_graph.add_dep(node, dep);
                if visited.insert(dep) {
                    stack.push(dep);
                }
            }
            for post in deps_conf.post_deps() {
                deps_graph.add_dep(post, node);
                if visited.insert(post) {
                    stack.push(post)
                }
            }
        }
        deps_graph
    }

    pub fn try_resolve(self) -> Result<impl Iterator<Item = &'a Identifier>, Box<dyn Error>> {
        Ok(self.deps_graph.into_iter()) // TODO: handle loops
    }
}
