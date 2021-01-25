use itertools::Itertools;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::iter;

pub struct DepsGraph<'a, I> {
    dependcies: HashMap<&'a I, DepsConf<&'a I>>,
    roots: HashSet<&'a I>,
}

impl<'a, I: Eq + Hash> DepsGraph<'a, I> {
    pub fn init(roots: impl IntoIterator<Item = &'a I>) -> Self {
        let roots: HashSet<&'a I> = roots.into_iter().collect();
        DepsGraph {
            dependcies: roots.iter().map(|&k| (k, DepsConf::new())).collect(),
            roots,
        }
    }

    pub fn new() -> Self {
        DepsGraph {
            dependcies: HashMap::new(),
            roots: HashSet::new(),
        }
    }

    pub fn build(
        roots: impl IntoIterator<Item = &'a I>,
        get_deps_for: impl Fn(&'a I) -> &'a DepsConf<I>,
    ) -> Self {
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

    pub fn add_node(&mut self, node: &'a I) {
        self.ensure_node(node);
    }

    fn ensure_node(&mut self, node: &'a I) -> &mut DepsConf<&'a I> {
        let deps = &mut self.dependcies;
        let roots = &mut self.roots;
        deps.entry(node).or_insert_with(|| {
            roots.insert(node);
            DepsConf::new()
        })
    }

    pub fn add_dep(&mut self, node: &'a I, dep: &'a I) {
        self.ensure_node(dep);
        self.ensure_node(node).add_dep(dep);
        self.roots.remove(dep);
    }

    pub fn add_post_dep(&mut self, node: &'a I, post_dep: &'a I) {
        self.ensure_node(post_dep);
        self.ensure_node(node).add_post_dep(post_dep);
        self.roots.remove(node);
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
                let deps_conf = &self.dependcies[&next];
                stack.extend(deps_conf.post_deps());
                stack.push(next);
                stack.extend(deps_conf.deps());
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

#[derive(Debug)]
pub struct DepsConf<I> {
    deps: HashSet<I>,
    post_deps: HashSet<I>,
}

impl<I: Eq + Hash> DepsConf<I> {
    pub fn new() -> Self {
        DepsConf {
            deps: HashSet::new(),
            post_deps: HashSet::new(),
        }
    }

    pub fn deps(&self) -> impl Iterator<Item = &I> {
        self.deps.iter()
    }

    pub fn post_deps(&self) -> impl Iterator<Item = &I> {
        self.post_deps.iter()
    }

    pub fn add_dep(&mut self, dep: I) {
        self.deps.insert(dep);
    }

    pub fn add_post_dep(&mut self, post_dep: I) {
        self.post_deps.insert(post_dep);
    }

    pub fn add_deps(&mut self, deps: impl IntoIterator<Item = I>) {
        self.deps.extend(deps)
    }

    pub fn add_post_deps(&mut self, post_deps: impl IntoIterator<Item = I>) {
        self.post_deps.extend(post_deps)
    }

    pub fn merge(&mut self, other: Self) {
        self.add_deps(other.deps);
        self.add_post_deps(other.post_deps);
    }

    pub fn remove_deps<'a>(&'a mut self, deps: impl IntoIterator<Item = &'a I>) {
        for dep in deps {
            self.deps.remove(dep);
        }
    }

    pub fn remove_post_deps<'a>(&'a mut self, post_deps: impl IntoIterator<Item = &'a I>) {
        for post_dep in post_deps {
            self.post_deps.remove(post_dep);
        }
    }

    pub fn disjoin(&mut self, other: &Self) {
        self.remove_deps(&other.deps);
        self.remove_post_deps(&other.post_deps);
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
}
