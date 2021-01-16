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

#[cfg(test)]
mod tests {
    use super::DepsGraph;

    #[test]
    fn empty() {
        assert_eq!(DepsGraph::<i32>::new().into_iter().count(), 0);
        assert_eq!(DepsGraph::<i32>::init(vec![]).into_iter().count(), 0);
    }

    #[test]
    fn no_deps() {
        for i in 1..100 {
            let mut resolved: Vec<_> = DepsGraph::init(0..i).into_iter().collect();
            resolved.sort();
            assert_eq!(resolved, (0..i).collect::<Vec<_>>());
        }
    }

    #[test]
    fn one_dep() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);

        let resolved: Vec<_> = graph.into_iter().collect();
        assert_eq!(resolved, vec![2, 1]);
    }

    #[test]
    fn two_deps_linear() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 3);

        let resolved: Vec<_> = graph.into_iter().collect();
        assert_eq!(resolved, vec![3, 2, 1]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(0, 1);

        let resolved: Vec<_> = graph.into_iter().collect();
        assert_eq!(resolved, vec![2, 1, 0]);
    }

    #[test]
    fn two_deps_for_one_node() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(1, 3);

        let resolved: Vec<_> = graph.into_iter().collect();
        let pos = |x| {
            resolved
                .iter()
                .position(|&y| y == x)
                .expect(&format!("{} is not presented", x))
        };

        assert!(pos(2) < pos(1));
        assert!(pos(3) < pos(1));
    }

    #[test]
    fn two_parallel_deps() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(3, 4);

        let resolved: Vec<_> = graph.into_iter().collect();
        let pos = |x| {
            resolved
                .iter()
                .position(|&y| y == x)
                .expect(&format!("{} is not presented", x))
        };

        assert!(pos(2) < pos(1));
        assert!(pos(4) < pos(3));
    }
}
