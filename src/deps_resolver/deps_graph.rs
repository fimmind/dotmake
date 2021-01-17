use itertools::{Itertools, Unique};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type DependentsSet<I> = HashSet<I>;
pub struct DepsGraph<I> {
    dependcies: HashMap<I, DependentsSet<I>>,
    roots: HashSet<I>,
}

impl<I: Copy + Eq + Hash> DepsGraph<I> {
    pub fn init(roots: impl IntoIterator<Item = I>) -> Self {
        let roots: HashSet<I> = roots.into_iter().collect();
        DepsGraph {
            dependcies: roots.iter().map(|&k| (k, HashSet::new())).collect(),
            roots,
        }
    }

    pub fn new() -> Self {
        DepsGraph {
            dependcies: HashMap::new(),
            roots: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: I) {
        if !self.dependcies.contains_key(&node) {
            self.roots.insert(node);
        }
    }

    pub fn add_dep(&mut self, node: I, dep: I) {
        self.add_node(node);
        self.roots.remove(&dep);
        self.dependcies.entry(node).or_default().insert(dep);
        self.dependcies.entry(dep).or_default();
    }

    pub fn find_loops(&self) -> Vec<Vec<I>> {
        vec![] // TODO: loops searching
    }
}

pub struct DepsIter<I> {
    graph: HashMap<I, HashSet<I>>,
    stack: Vec<I>,
    resolved: HashSet<I>,
}

impl<I: Copy + Eq + Hash> Iterator for DepsIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.stack.pop() {
            if self.resolved.contains(&next) {
                return Some(next);
            }
            self.stack.push(next);
            self.stack.extend(&self.graph[&next]);
            self.resolved.insert(next);
        }
        None
    }
}

impl<I: Copy + Eq + Hash> IntoIterator for DepsGraph<I> {
    type Item = I;

    type IntoIter = Unique<DepsIter<I>>;

    fn into_iter(self) -> Self::IntoIter {
        DepsIter {
            stack: self.roots.iter().map(|r| *r).collect(),
            graph: self.dependcies,
            resolved: HashSet::new(),
        }
        .unique()
    }
}

#[cfg(test)]
mod tests {
    use super::DepsGraph;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn roots() {
        for i in 1..100 {
            let mut roots: Vec<_> = DepsGraph::init(0..i).roots.into_iter().collect();
            roots.sort();
            assert_eq!(roots, (0..i).collect::<Vec<_>>());
        }

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        let roots: Vec<_> = graph.roots.into_iter().collect();
        assert_eq!(roots, vec![1]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 3);
        let roots: Vec<_> = graph.roots.into_iter().collect();
        assert_eq!(roots, vec![1]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(3, 4);
        assert!(graph.roots.len() == 2);
        assert!(graph.roots.contains(&1));
        assert!(graph.roots.contains(&3));
    }

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

    fn test_resolving(edges: Vec<(i32, i32)>) {
        let mut graph = DepsGraph::new();
        let mut nodes = HashSet::new();
        for &(node, dep) in edges.iter() {
            graph.add_dep(node, dep);
            nodes.insert(node);
            nodes.insert(dep);
        }

        let resolved: Vec<_> = graph.into_iter().collect();
        println!("Resolved: {:?}", resolved);
        assert_eq!(
            resolved.len(),
            nodes.len(),
            "{} nodes inserted, but {} resolved",
            nodes.len(),
            resolved.len()
        );

        let positions: HashMap<_, _> = resolved.into_iter().enumerate().map(|(i, d)| (d, i)).collect();
        for &(node, dep) in edges.iter() {
            assert!(
                positions[&dep] < positions[&node],
                "pos({0}) < pos({1}), but {0} depends on {1}",
                node,
                dep
            );
        }
    }

    #[test]
    fn one_dep() {
        test_resolving(vec![(1, 2)]);
        test_resolving(vec![(2, 1)]);
    }

    #[test]
    fn two_simple_deps() {
        test_resolving(vec![(1, 2), (2, 3)]);
        test_resolving(vec![(1, 2), (0, 1)]);
        test_resolving(vec![(1, 2), (1, 3)]);
        test_resolving(vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn multiple_nodes_with_commond_dep() {
        test_resolving(vec![(1, 3), (2, 3)]);
        test_resolving(vec![(1, 3), (1, 2), (2, 3)]);
        test_resolving(vec![(1, 4), (2, 4), (3, 4)]);
        test_resolving(vec![(1, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
        test_resolving(vec![(1, 2), (2, 3), (1, 3), (3, 4)]);
    }

    #[test]
    fn find_no_loops() {
        for i in 1..100 {
            let graph = DepsGraph::init(0..i);
            assert!(graph.find_loops().is_empty());
        }

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(1, 3);
        assert!(graph.find_loops().is_empty());

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(3, 4);
        assert!(graph.find_loops().is_empty());

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 3);
        assert!(graph.find_loops().is_empty());

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(0, 1);
        assert!(graph.find_loops().is_empty());

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(1, 3);
        graph.add_dep(2, 3);
        assert!(graph.find_loops().is_empty());
    }

    #[test]
    fn find_single_loop() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 1);
        assert_eq!(graph.find_loops(), vec![vec![1, 1]]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 1);
        assert_eq!(graph.find_loops(), vec![vec![1, 2, 1]]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 3);
        graph.add_dep(3, 1);
        graph.add_dep(1, 4);
        graph.add_dep(1, 5);
        assert_eq!(graph.find_loops(), vec![vec![1, 2, 3, 1]]);
    }

    #[test]
    fn find_multiple_loops() {
        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 1);
        graph.add_dep(3, 3);
        graph.add_dep(4, 5);
        graph.add_dep(5, 4);

        let loops = graph.find_loops();
        println!("Found loops: {:?}", loops);
        assert_eq!(loops.len(), 3);
        assert!(loops.contains(&vec![1, 2, 1]));
        assert!(loops.contains(&vec![3, 3]));
        assert!(loops.contains(&vec![4, 5, 4]));
    }
}
