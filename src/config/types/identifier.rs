use serde::de::{self, Deserialize, Deserializer, Error};
use std::collections::HashSet;
use std::fmt;
use std::ops::Deref;
use std::str;

/// Represents a string containing no whitespace
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier(String);

impl Identifier {
    /// Creates new identifier with a given `name`.
    /// Return `Err(name)` if there is any whitespace in `name`
    pub fn new(name: String) -> Result<Self, String> {
        if name.chars().any(|ch| ch.is_whitespace()) {
            return Err(name);
        }
        Ok(Identifier(name))
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        if name.chars().any(|ch| ch.is_whitespace()) {
            Err(Error::invalid_value(
                de::Unexpected::Str(&name),
                &"a string without whitespace",
            ))
        } else {
            Ok(Identifier(name))
        }
    }
}

impl Deref for Identifier {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl str::FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::new(s.into()).map_err(|name| format!("invalid identifier: {}", name))
    }
}

/// Represents a whitespace-separated list of `Identifier`s
#[derive(Debug, Default, Clone)]
pub struct IdentifiersList(Vec<Identifier>);

impl IdentifiersList {
    fn new() -> Self {
        IdentifiersList(vec![])
    }
}

impl<'de> Deserialize<'de> for IdentifiersList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(IdentifiersList(
            String::deserialize(deserializer)?
                .split_whitespace()
                .map(|name| Identifier(name.to_string()))
                .collect(),
        ))
    }
}

impl Deref for IdentifiersList {
    type Target = Vec<Identifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a whitespace-separated set of `Identifier`s containing no duplicates
#[derive(Debug, Default, Clone)]
pub struct IdentifiersSet(HashSet<Identifier>);

impl IdentifiersSet {
    fn new() -> Self {
        IdentifiersSet(HashSet::new())
    }
}

impl<'de> Deserialize<'de> for IdentifiersSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut set = HashSet::new();
        for name in String::deserialize(deserializer)?.split_whitespace() {
            if !set.insert(Identifier(name.to_string())) {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Str(name),
                    &"a set of identifiers without duplicates",
                ));
            }
        }
        Ok(IdentifiersSet(set))
    }
}

impl Deref for IdentifiersSet {
    type Target = HashSet<Identifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
