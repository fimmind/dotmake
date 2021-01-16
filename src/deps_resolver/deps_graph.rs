use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type DependentsSet<I> = HashSet<I>;
pub struct DepsGraph<I> {
    dependent_nodes: HashMap<I, DependentsSet<I>>,
}

impl<I: Copy + Eq + Hash> DepsGraph<I> {
    pub fn init(roots: impl IntoIterator<Item = I>) -> Self {
        DepsGraph {
            dependent_nodes: roots.into_iter().map(|k| (k, HashSet::new())).collect(),
        }
    }

    pub fn new() -> Self {
        DepsGraph {
            dependent_nodes: HashMap::new(),
        }
    }

    fn get_leafs(&self) -> impl Iterator<Item = I> + '_ {
        self.dependent_nodes
            .iter()
            .filter(|(_k, v)| v.is_empty())
            .map(|(k, _v)| *k)
    }

    pub fn add_dep(&mut self, node: I, dep: I) {
        self.dependent_nodes.entry(node).or_default().insert(dep);
        self.dependent_nodes.entry(dep).or_default();
    }

    pub fn find_loops(&self) -> Vec<Vec<I>> {
        vec![] // TODO: loops searching
    }
}

pub struct DepsIter<I> {
    graph: HashMap<I, HashSet<I>>,
    stack: Vec<I>,
}

impl<I: Copy + Eq + Hash> Iterator for DepsIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|next| {
            self.stack.extend(&self.graph[&next]);
            next
        })
    }
}

impl<I: Copy + Eq + Hash> IntoIterator for DepsGraph<I> {
    type Item = I;

    type IntoIter = DepsIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        DepsIter {
            stack: self.get_leafs().collect(),
            graph: self.dependent_nodes,
        }
    }
}
