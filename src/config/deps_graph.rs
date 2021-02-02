//! Dependencies graph abstranction
//!
//! # Examples
//! ```
//! let graph: DepsGraph<_> = hashmap! {
//!     1 => hashset!{2, 3},
//!     2 => hashset!{3, 4},
//!     3 => hashset!{4},
//!     4 => hashset!{5},
//!     7 => hashset!{1, 2, 5}
//! }.into();
//! let roots = vec![&1];
//! assert_eq!(graph.resolve(roots), Ok(vec![&5, &4, &3, &2, &1]));
//! ```

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;

/// Simple dependencies graph abstraction that provides a convinient way of
/// dependencies resolution
pub struct DepsGraph<I> {
    graph: HashMap<I, HashSet<I>>,
}

impl<I: Hash + Eq + Debug + Display + Clone> DepsGraph<I> {
    /// Resolve dependencies for given roots
    ///
    /// This will collect all direct and indirect dependencies for every node in
    /// `roots` and return them in an order such that for every node of
    /// resulting vector it's dependencies, if any, are placed before that node.
    /// It's guaranteed that resulting vector contains no duplicates
    ///
    /// # Errors
    /// If it's not possible to complete resolving due to cycle found in
    /// dependencies, [`CycleError`] is returned
    ///
    /// [`CycleError`]: ./struct.CycleError.html
    pub fn resolve<'a>(&'a self, roots: Vec<&'a I>) -> Result<Vec<&'a I>, CycleError<I>> {
        let mut res = Vec::new();
        let mut resolving = HashSet::new();
        let mut resolved = HashSet::new();
        let mut stack = roots;
        stack.reverse();

        while let Some(ident) = stack.pop() {
            if !resolved.contains(&ident) {
                match self.graph.get(&ident) {
                    Some(deps) => {
                        if deps.iter().all(|i| resolved.contains(i)) {
                            res.push(ident);
                            resolved.insert(ident);
                            resolving.remove(&ident);
                        } else if resolving.contains(&ident) {
                            Err(CycleError::new(
                                self.find_path(&ident, &ident).unwrap().own_nodes(),
                            ))?;
                        } else {
                            stack.push(ident);
                            stack.extend(deps.iter());
                            resolving.insert(ident);
                        }
                    }
                    None => {
                        resolved.insert(ident);
                        res.push(ident);
                    }
                }
            }
        }

        Ok(res)
    }

    /// Finds the shortest path from `start` to `dest` in dependencies graph if
    /// one exists. Remember that edges of the graph are orientated from nodes
    /// to their dependencies
    ///
    /// First and last elements of the resulting vector are always `start` and
    /// `dest` respectively
    fn find_path(&self, start: &I, dest: &I) -> Option<Path<&I>> {
        todo!("DepsGraph::find_path")
    }
}

impl<'a, I> From<HashMap<I, HashSet<I>>> for DepsGraph<I> {
    fn from(graph: HashMap<I, HashSet<I>>) -> Self {
        DepsGraph { graph }
    }
}

/// An error stating that a dependencies graph contains a cycle and thus can't be
/// resolved
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CycleError<I> {
    path: Path<I>,
}

impl<I: Eq> CycleError<I> {
    /// Create a new `CycleError`. Panics if `path.first() != path.last()` or if
    /// `path.len() < 2`
    pub fn new(path: Path<I>) -> Self {
        assert!(path.first() == path.last());
        assert!(path.len() > 1);
        Self { path }
    }

    /// Get a path of the cycle
    pub fn path(&self) -> &Path<I> {
        &self.path
    }
}

impl<I: Debug + Display> Error for CycleError<I> {}
impl<I: Display> Display for CycleError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Found cycle in dependencies graph: {}", self.path)
    }
}

impl<I> From<CycleError<I>> for Path<I> {
    fn from(err: CycleError<I>) -> Self {
        err.path
    }
}

/// A path in dependencies graph
///
/// 1. It ensures that a path contains at least one item
/// 2. It provides pretty-printing `Display` implementation
///
/// # Examples
/// ```
/// let path = Path::new(vec![1, 2, 3]);
/// assert_eq!(path.to_string(), "1 -> 2 -> 3");
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Path<I> {
    path: Vec<I>,
}

impl<I> Path<I> {
    /// Create a new path. Panics if `path` is empty
    pub fn new(path: Vec<I>) -> Self {
        assert!(!path.is_empty());
        Path { path }
    }
}

impl<I: ToOwned> Path<&I> {
    /// Create a new path replacing every node with it's owned equivalent
    pub fn own_nodes(&self) -> Path<I::Owned> {
        self.iter().map(|&n| n.to_owned()).collect()
    }
}

impl<I> IntoIterator for Path<I> {
    type Item = I;

    type IntoIter = <Vec<I> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<I> FromIterator<I> for Path<I> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Path::new(iter.into_iter().collect())
    }
}

impl<I: Display> Display for Path<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes = self.iter();
        write!(f, "{}", nodes.next().unwrap())?;
        while let Some(node) = nodes.next() {
            write!(f, " -> {}", node)?;
        }
        Ok(())
    }
}

impl<I> From<Path<I>> for Vec<I> {
    fn from(path: Path<I>) -> Self {
        path.path
    }
}

impl<T> Deref for Path<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::{DepsGraph, Path};
    use itertools::Itertools;
    use maplit::{hashmap, hashset};
    use std::collections::HashMap;
    use std::collections::HashSet;

    /// Construct a new dependencies graph
    fn deps_graph(graph: HashMap<i32, HashSet<i32>>) -> DepsGraph<i32> {
        graph.into()
    }

    /// Resolve a given graph starting with given roots and then assert that for
    /// every (node, dep) pair from that graph position(dep) < position(node)
    fn test_resolving(roots: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
        let deps_graph = deps_graph(graph.clone());
        let resolved = deps_graph.resolve(roots.iter().collect()).unwrap();
        println!("Resolved: {:?}", resolved);

        let positions: HashMap<_, _> = resolved.iter().map(|&&i| i).zip(0..).collect();
        for (node, deps) in graph {
            for dep in deps {
                if let (Some(node_pos), Some(dep_pos)) = (positions.get(node), positions.get(dep)) {
                    assert!(
                        dep_pos < node_pos,
                        "pos({0}) ≤ pos({1}), but {0} depends on {1}",
                        node,
                        dep
                    );
                }
            }
        }

        resolved.into_iter().map(|&i| i).collect()
    }

    #[test]
    fn empty_roots() {
        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3},
        };
        assert!(test_resolving(&[], &deps_graph).is_empty());
    }

    #[test]
    fn no_deps() {
        let deps_graph = hashmap! {};
        for i in 0..100 {
            let roots = (0..i).collect_vec();
            let mut resolved = test_resolving(&roots, &deps_graph);
            resolved.sort();
            assert_eq!(resolved, roots);
        }
    }

    #[test]
    fn one_dep() {
        let deps_graph = hashmap! {
            1 => hashset!{2}
        };
        assert_eq!(vec![2, 1], test_resolving(&[1], &deps_graph,),);
        assert_eq!(vec![2], test_resolving(&[2], &deps_graph,))
    }

    #[test]
    fn two_simple_deps() {
        let deps_graph = &hashmap! {
            1 => hashset!{2},
            2 => hashset!{3},
        };
        assert_eq!(test_resolving(&[1], &deps_graph), vec![3, 2, 1]);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![3, 2]);
        assert_eq!(test_resolving(&[1, 2], &deps_graph), vec![3, 2, 1]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![3]);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
        };
        test_resolving(&[1], &deps_graph);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![3]);

        let deps_graph = hashmap! {
            1 => hashset!{2},
            3 => hashset!{4},
        };
        test_resolving(&[1, 3], &deps_graph);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![4, 3]);
    }

    #[test]
    fn multiple_nodes_with_commond_dep() {
        let deps_graph = hashmap! {
            1 => hashset!{3},
            2 => hashset!{3},
        };
        test_resolving(&[1, 2], &deps_graph);
        assert_eq!(test_resolving(&[1], &deps_graph), vec![3, 1]);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![3, 2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![3]);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3},
        };
        assert_eq!(test_resolving(&[1], &deps_graph), vec![3, 2, 1]);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![3, 2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![3]);
        assert_eq!(test_resolving(&[1, 2], &deps_graph), vec![3, 2, 1]);
        assert_eq!(test_resolving(&[2, 3], &deps_graph), vec![3, 2]);
        assert_eq!(test_resolving(&[1, 3], &deps_graph), vec![3, 2, 1]);
        assert_eq!(test_resolving(&[1, 2, 3], &deps_graph), vec![3, 2, 1]);

        let deps_graph = hashmap! {
            1 => hashset!{4},
            2 => hashset!{4},
            3 => hashset!{4},
        };
        test_resolving(&[1, 2, 3], &deps_graph);
        test_resolving(&[1, 2], &deps_graph);
        test_resolving(&[2, 3], &deps_graph);
        test_resolving(&[1, 3], &deps_graph);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3, 4},
            3 => hashset!{4},
        };
        assert_eq!(test_resolving(&[1], &deps_graph), vec![4, 3, 2, 1]);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![4, 3, 2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![4, 3]);
        assert_eq!(test_resolving(&[4], &deps_graph), vec![4]);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3},
            3 => hashset!{4},
        };
        assert_eq!(test_resolving(&[1], &deps_graph), vec![4, 3, 2, 1]);
        assert_eq!(test_resolving(&[2], &deps_graph), vec![4, 3, 2]);
        assert_eq!(test_resolving(&[3], &deps_graph), vec![4, 3]);
        assert_eq!(test_resolving(&[4], &deps_graph), vec![4]);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3, 4},
            4 => hashset!{5},
        };
        assert_eq!(5, test_resolving(&[1], &deps_graph).len());
        assert_eq!(5, test_resolving(&[1, 3], &deps_graph).len());
        assert_eq!(5, test_resolving(&[3, 1], &deps_graph).len());
        assert_eq!(test_resolving(&[3], &deps_graph), vec![3]);

        let deps_graph = hashmap! {
            1 => hashset!{2, 3},
            2 => hashset!{3, 4},
            4 => hashset!{5},
            7 => hashset!{2, 5}
        };
        assert_eq!(5, test_resolving(&[1], &deps_graph).len());
        assert_eq!(6, test_resolving(&[1, 7], &deps_graph).len());
        assert_eq!(5, test_resolving(&[7], &deps_graph).len());
    }

    /// Assert that a graph has a cycle and then return that cycle path
    fn test_single_cycle(roots: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
        let deps_graph = deps_graph(graph.clone());
        let cycle_path: Path<i32> = deps_graph
            .resolve(roots.iter().collect())
            .unwrap_err()
            .into();

        println!("Found cycle: {}", cycle_path);
        cycle_path.into()
    }

    #[test]
    fn find_path() {
        let graph = deps_graph(hashmap! {
            1 => hashset!{2},
        });
        assert_eq!(graph.find_path(&1, &2), Some(Path::new(vec![&1, &2])));
        assert_eq!(graph.find_path(&1, &1), None);

        let graph = deps_graph(hashmap! {
            1 => hashset!{2},
            2 => hashset!{1},
        });
        assert_eq!(graph.find_path(&1, &1), Some(Path::new(vec![&1, &2, &1])));
        assert_eq!(graph.find_path(&2, &2), Some(Path::new(vec![&2, &1, &2])));
        assert_eq!(graph.find_path(&1, &2), Some(Path::new(vec![&1, &2])));
        assert_eq!(graph.find_path(&2, &1), Some(Path::new(vec![&2, &1])));

        let graph = deps_graph(hashmap! {
            1 => hashset!{2, 4},
            2 => hashset!{3},
            3 => hashset!{5},
            4 => hashset!{3, 5},
            5 => hashset!{2},
            6 => hashset!{2, 3},
        });
        assert_eq!(
            graph.find_path(&1, &2),
            Some(Path::new(vec![&1, &3, &5, &2]))
        );
        assert_eq!(
            graph.find_path(&1, &2),
            Some(Path::new(vec![&1, &3, &5, &2]))
        );
        assert_eq!(
            graph.find_path(&3, &3),
            Some(Path::new(vec![&3, &5, &2, &3]))
        );
        assert_eq!(
            graph.find_path(&3, &3),
            Some(Path::new(vec![&3, &5, &2, &3]))
        );
        assert_eq!(
            graph.find_path(&5, &5),
            Some(Path::new(vec![&5, &2, &3, &5]))
        );
        assert_eq!(graph.find_path(&1, &6), None);
        assert_eq!(graph.find_path(&6, &4), None);
    }

    #[test]
    fn display_path() {
        let path = Path::new(vec![1]);
        assert_eq!(path.to_string(), "1");

        let path = Path::new(vec![1, 2, 3]);
        assert_eq!(path.to_string(), "1 -> 2 -> 3");
    }
}
