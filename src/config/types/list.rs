use serde::de::{Deserialize, Deserializer};
use std::ops::{Deref, DerefMut};

/// A helper for deserializing `List` as either a sequence or a singletone
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ListEnum<T> {
    Sequence(Vec<T>),
    Singletone(T),
}

impl<T> ListEnum<T> {
    fn to_list(self) -> List<T> {
        match self {
            ListEnum::Sequence(vec) => List { vec },
            ListEnum::Singletone(elem) => List { vec: vec![elem] },
        }
    }
}


/// A structure representing a sequence of items with a special way of deserializing: instead of
/// only accepting a sequence of items, it also accepts a single value, witch is treated as a
/// singletone sequence containing this value
#[derive(Debug)]
pub struct List<T> {
    vec: Vec<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.vec
    }

    pub fn to_vec(self) -> Vec<T> {
        self.vec
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { vec: vec![] }
    }
}

impl<'de, T> Deserialize<'de> for List<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ListEnum::deserialize(deserializer).map(ListEnum::to_list)
    }
}

impl<T> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<T> Into<Vec<T>> for List<T> {
    fn into(self) -> Vec<T> {
        self.to_vec()
    }
}
