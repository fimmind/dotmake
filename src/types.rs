//! Various helper types

use serde::de::{self, Deserialize, Deserializer, Error};
use std::fmt;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str;
use std::str::FromStr;

/// A structure representing a string containing no whitespace
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier(String);

impl Identifier {
    /// Creates new identifier with a given `name`.
    ///
    /// # Errors
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
            ))?;
        }
        Ok(Identifier(name))
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

/// A structure representing a list of identifers separeted with whitespace
#[derive(Debug, Deserialize)]
pub struct Identifiers(String);

impl IntoIterator for &Identifiers {
    type Item = Identifier;
    type IntoIter = impl Iterator<Item = Identifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.split_whitespace().map(|s| Identifier(s.to_owned()))
    }
}

/// Path that automatically expands tidle using [`shellexpand::tilde`]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct UserPath {
    path: PathBuf,
}

impl<'de> Deserialize<'de> for UserPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let path = PathBuf::from(shellexpand::tilde(&s).to_string());

        Ok(UserPath { path })
    }
}

impl From<UserPath> for PathBuf {
    fn from(path: UserPath) -> Self {
        path.path
    }
}

impl Deref for UserPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for UserPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl Hash for UserPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

impl FromStr for UserPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserPath {
            path: PathBuf::from_str(s)?,
        })
    }
}

impl AsRef<Path> for UserPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
