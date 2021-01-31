use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::FromIterator;

pub struct DepsGraph<I> {
    graph: HashMap<I, HashSet<I>>,
}

impl<I: Hash + Eq + Debug + Display + Clone> DepsGraph<I> {
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

    pub fn find_path(&self, start: &I, dest: &I) -> Option<Path<&I>> {
        todo!("DepsGraph::find_path")
    }
}

impl<'a, I> From<HashMap<I, HashSet<I>>> for DepsGraph<I> {
    fn from(graph: HashMap<I, HashSet<I>>) -> Self {
        DepsGraph { graph }
    }
}

#[derive(Debug)]
pub struct CycleError<I> {
    path: Path<I>,
}

impl<I> CycleError<I> {
    pub fn new(path: Path<I>) -> Self {
        Self { path }
    }
}

impl<I: Debug + Display> Error for CycleError<I> {}
impl<I: Display> Display for CycleError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Found cycle in dependencies graph: {}", self.path)
    }
}

#[derive(Debug)]
pub struct Path<I> {
    path: Vec<I>,
}

impl<I> Path<I> {
    pub fn new(path: Vec<I>) -> Self {
        Path { path }
    }

    pub fn iter(&self) -> impl Iterator<Item = &I> {
        self.path.iter()
    }
}

impl<I: ToOwned> Path<&I> {
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
        if let Some(node) = nodes.next() {
            write!(f, "{}", node)?;
        }
        while let Some(node) = nodes.next() {
            write!(f, " -> {}", node)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DepsGraph;
    use itertools::Itertools;
    use maplit::{hashmap, hashset};
    use std::collections::HashMap;
    use std::collections::HashSet;

    /// Resolve a given graph starting with given roots and then assert that for
    /// every (node, dep) pair from that graph position(dep) < position(node)
    fn test_resolving(roots: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
        let deps_graph: DepsGraph<i32> = graph.clone().into();
        let resolved = deps_graph.resolve(roots.iter().collect()).unwrap();
        println!("Resolved: {:?}", resolved);

        let positions: HashMap<_, _> = resolved.iter().map(|&&i| i).zip(0..).collect();
        for (node, deps) in graph {
            for dep in deps {
                if let (Some(node_pos), Some(dep_pos)) = (positions.get(node), positions.get(dep))
                {
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
    }
}
