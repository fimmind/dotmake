use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type DependentsSet<I> = HashSet<I>;
pub struct DepsGraph<I> {
    dependent_nodes: HashMap<I, DependentsSet<I>>,
    roots: HashSet<I>,
}

impl<I: Copy + Eq + Hash> DepsGraph<I> {
    pub fn init(roots: impl IntoIterator<Item = I>) -> Self {
        let roots: HashSet<I> = roots.into_iter().collect();
        DepsGraph {
            dependent_nodes: roots.iter().map(|&k| (k, HashSet::new())).collect(),
            roots,
        }
    }

    pub fn new() -> Self {
        DepsGraph {
            dependent_nodes: HashMap::new(),
            roots: HashSet::new(),
        }
    }

    pub fn add_dep(&mut self, node: I, dep: I) {
        self.roots.remove(&node);
        if !self.dependent_nodes.contains_key(&dep) {
            self.roots.insert(dep);
        }
        self.dependent_nodes.entry(dep).or_default().insert(node);
        self.dependent_nodes.entry(node).or_default();
    }

    pub fn find_loops(&self) -> Vec<Vec<I>> {
        vec![] // TODO: loops searching
    }
}

pub struct DepsIter<I> {
    graph: HashMap<I, HashSet<I>>,
    queue: VecDeque<I>,
    visited: HashSet<I>,
}

impl<I: Copy + Eq + Hash> Iterator for DepsIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|next| {
            for &dependent in &self.graph[&next] {
                if self.visited.insert(dependent) {
                    self.queue.push_back(dependent);
                }
            }
            next
        })
    }
}

impl<I: Copy + Eq + Hash> IntoIterator for DepsGraph<I> {
    type Item = I;

    type IntoIter = DepsIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        DepsIter {
            queue: self.roots.iter().map(|r| *r).collect(),
            graph: self.dependent_nodes,
            visited: self.roots.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DepsGraph;

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
        assert_eq!(roots, vec![2]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(2, 3);
        let roots: Vec<_> = graph.roots.into_iter().collect();
        assert_eq!(roots, vec![3]);

        let mut graph = DepsGraph::<i32>::new();
        graph.add_dep(1, 2);
        graph.add_dep(3, 4);
        assert!(graph.roots.len() == 2);
        assert!(graph.roots.contains(&2));
        assert!(graph.roots.contains(&4));
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

        println!("{:?}", resolved);
        assert_eq!(resolved.len(), 3);
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

        println!("{:?}", resolved);
        assert_eq!(resolved.len(), 4);
        assert!(pos(2) < pos(1));
        assert!(pos(4) < pos(3));
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
