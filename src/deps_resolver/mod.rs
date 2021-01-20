mod deps_graph;

use crate::identifier::Identifier;
use deps_graph::DepsGraph;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct DepsConf {
    deps: HashSet<Identifier>,
    post_deps: HashSet<Identifier>,
}

impl DepsConf {
    pub fn new() -> Self {
        DepsConf {
            deps: HashSet::new(),
            post_deps: HashSet::new(),
        }
    }

    pub fn deps(&self) -> impl Iterator<Item = &Identifier> {
        self.deps.iter()
    }

    pub fn post_deps(&self) -> impl Iterator<Item = &Identifier> {
        self.post_deps.iter()
    }

    pub fn add_deps<I>(&mut self, deps: I)
    where
        I: IntoIterator<Item = Identifier>
    {
        self.deps.extend(deps)
    }

    pub fn add_post_deps<I>(&mut self, post_deps: I)
    where
        I: IntoIterator<Item = Identifier>
    {
        self.post_deps.extend(post_deps)
    }

    pub fn merge(&mut self, other: Self) {
        self.add_deps(other.deps);
        self.add_post_deps(other.post_deps);
    }

    pub fn remove_deps<'a, I>(&mut self, deps: I)
    where
        I: IntoIterator<Item = &'a Identifier>
    {
        for dep in deps {
            self.deps.remove(dep);
        }
    }

    pub fn remove_post_deps<'a, I>(&mut self, post_deps: I)
    where
        I: IntoIterator<Item = &'a Identifier>
    {
        for post_dep in post_deps {
            self.post_deps.remove(post_dep);
        }
    }

    pub fn disjoin(&mut self, other: &Self) {
        self.remove_deps(&other.deps);
        self.remove_post_deps(&other.post_deps);
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
