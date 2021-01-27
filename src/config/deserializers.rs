use serde::de::{Deserialize, Deserializer};

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
pub fn deserialize_list<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    ListEnum::deserialize(deserializer).map(ListEnum::to_vec)
}
