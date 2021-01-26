use itertools::Itertools;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter;

pub struct DepsGraph<'a, I> {
    dependcies: HashMap<&'a I, HashSet<&'a I>>,
    roots: HashSet<&'a I>,
}

impl<'a, I: Debug + Eq + Hash> DepsGraph<'a, I> {
    pub fn init(roots: impl IntoIterator<Item = &'a I>) -> Self {
        let roots: HashSet<&'a I> = roots.into_iter().collect();
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

    pub fn build(roots: Vec<&'a I>, get_deps_for: impl Fn(&'a I) -> &'a HashSet<I>) -> Self {
        let mut stack = roots;
        let mut seen = HashSet::new();
        let mut deps_graph = DepsGraph::new();

        while let Some(node) = stack.pop() {
            let deps = get_deps_for(node);
            deps_graph.add_deps(node, deps);
            for dep in deps {
                if seen.insert(dep) {
                    stack.push(dep);
                }
            }
        }
        deps_graph
    }

    pub fn add_node(&mut self, node: &'a I) {
        self.ensure_node(node);
    }

    fn ensure_node(&mut self, node: &'a I) -> &mut HashSet<&'a I> {
        let deps = &mut self.dependcies;
        let roots = &mut self.roots;
        deps.entry(node).or_insert_with(|| {
            roots.insert(node);
            HashSet::new()
        })
    }

    pub fn add_dep(&mut self, node: &'a I, dep: &'a I) {
        self.ensure_node(dep);
        self.ensure_node(node).insert(dep);
        self.roots.remove(dep);
    }

    pub fn add_deps(&mut self, node: &'a I, deps: impl IntoIterator<Item = &'a I>) {
        for dep in deps {
            self.add_dep(node, dep);
        }
    }

    fn find_path(&self, from: &'a I, to: &'a I) -> Option<Vec<&'a I>> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = Result<&'a I, &'a I>> + '_ {
        let mut stack = self.roots.iter().map(|&n| n).collect_vec();
        let mut resolved = HashSet::new();

        iter::from_fn(move || {
            while let Some(next) = stack.pop() {
                if resolved.contains(&next) {
                    return Some(Ok(next));
                }
                stack.push(next);
                stack.extend(&self.dependcies[&next]);
                resolved.insert(next);
            }
            None
        })
        .unique()
    }

    pub fn resolve(&self) -> Result<Vec<&'a I>, Cycle<'a, I>> {
        let mut res = Vec::with_capacity(self.dependcies.keys().len());
        for next in self.iter() {
            match next {
                Ok(node) => res.push(node),
                Err(cycled_node) => {
                    return Err(Cycle::new(
                        self.find_path(cycled_node, cycled_node).unwrap(),
                    ))
                }
            }
        }
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Cycle<'a, I> {
    cycle: Vec<&'a I>,
}

impl<'a, I> Cycle<'a, I> {
    pub fn new(cycle: Vec<&'a I>) -> Self {
        Self { cycle }
    }
}

impl<'a, I: Display> Display for Cycle<'a, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(first) = self.cycle.first() {
            write!(f, "{}", first)?;
        }
        for next in self.cycle.iter().skip(1) {
            write!(f, " -> {}", next)?;
        }
        Ok(())
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
            let nodes = (0..i).collect::<Vec<_>>();
            let mut roots: Vec<_> = DepsGraph::init(&nodes)
                .roots
                .into_iter()
                .map(|&n| n)
                .collect();
            roots.sort();
            assert_eq!(roots, nodes);
        }

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(&1, &2);
        let roots: Vec<_> = graph.roots.into_iter().collect();
        assert_eq!(roots, vec![&1]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(&1, &2);
        graph.add_dep(&2, &3);
        let roots: Vec<_> = graph.roots.into_iter().collect();
        assert_eq!(roots, vec![&1]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(&1, &2);
        graph.add_dep(&3, &4);
        assert!(graph.roots.len() == 2);
        assert!(graph.roots.contains(&&1));
        assert!(graph.roots.contains(&&3));
    }

    #[test]
    fn empty() {
        assert_eq!(DepsGraph::<i32>::new().resolve().unwrap().len(), 0);
        assert_eq!(DepsGraph::<i32>::init(vec![]).resolve().unwrap().len(), 0);
    }

    #[test]
    fn no_deps() {
        for i in 1..100 {
            let nodes = (0..i).collect::<Vec<_>>();
            let mut resolved: Vec<_> = DepsGraph::init(&nodes).resolve().unwrap();
            resolved.sort();
            assert!(resolved.into_iter().eq(nodes.iter()));
        }
    }

    fn test_resolving(edges: Vec<(i32, i32)>) {
        let mut graph = DepsGraph::new();
        let mut nodes = HashSet::new();
        for (node, dep) in edges.iter() {
            graph.add_dep(node, dep);
            nodes.insert(node);
            nodes.insert(dep);
        }

        let resolved: Vec<_> = graph.resolve().unwrap();
        println!("Resolved: {:?}", resolved);
        assert_eq!(
            resolved.len(),
            nodes.len(),
            "{} nodes inserted, but {} resolved",
            nodes.len(),
            resolved.len()
        );

        let positions: HashMap<_, _> = resolved
            .into_iter()
            .enumerate()
            .map(|(i, d)| (d, i))
            .collect();
        for (node, dep) in edges.iter() {
            assert!(
                positions[dep] < positions[node],
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
}
