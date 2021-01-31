use std::ops::{Deref, DerefMut};

/// A helper for deserializing `List` as either a sequence or a singletone
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ListEnum<T> {
    Sequence(Vec<T>),
    Singletone(T),
}

impl<T> ListEnum<T> {
    fn to_vec(self) -> Vec<T> {
        match self {
            ListEnum::Sequence(vec) => vec,
            ListEnum::Singletone(elem) => vec![elem],
        }
    }
}

/// Deserialize a vector, but instead of only accepting a sequence of items, it
/// also accepts a single value, witch is treated as a singletone sequence
/// containing this value
#[derive(Debug, Deserialize)]
#[serde(from = "ListEnum<T>")]
pub struct List<T> {
    elems: Vec<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List::default()
    }
}

impl<T> From<ListEnum<T>> for List<T> {
    fn from(list_enum: ListEnum<T>) -> Self {
        List {
            elems: list_enum.to_vec(),
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(elems: Vec<T>) -> Self {
        List { elems }
    }
}

impl<T> Deref for List<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.elems
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elems
    }
}

impl<T> From<List<T>> for Vec<T> {
    fn from(list: List<T>) -> Self {
        list.elems
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { elems: vec![] }
    }
}
