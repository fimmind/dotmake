use crate::identifier::Identifier;
use serde::de::{Deserialize, Deserializer};
use std::collections::HashSet;

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
pub fn list<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    ListEnum::deserialize(deserializer).map(ListEnum::to_vec)
}

/// Deserialize a whitespace-separated list of `Identifier`s
pub fn identifiers_list<'de, D>(deserializer: D) -> Result<Vec<Identifier>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split_whitespace()
        .map(|name| Identifier::new(name.to_string()).unwrap())
        .collect())
}

/// Deserialize a whitespace-separated set of `Identifier`s containing no duplicates
pub fn identifiers_set<'de, D>(deserializer: D) -> Result<HashSet<Identifier>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split_whitespace()
        .map(|name| Identifier::new(name.to_string()).unwrap())
        .collect())
}
