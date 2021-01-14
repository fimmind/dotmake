use crate::deserializers::identifiers_set;
use crate::identifier::Identifier;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DepsConf {
    #[serde(default, deserialize_with = "identifiers_set")]
    pub deps: HashSet<Identifier>,

    #[serde(default, deserialize_with = "identifiers_set")]
    pub post: HashSet<Identifier>,

    #[serde(default, deserialize_with = "identifiers_set")]
    pub synonyms: HashSet<Identifier>,
}

pub struct DepsResolver<'a, F> {
    roots: HashSet<&'a Identifier>,
    get_deps_for: F,
}

impl<'a, F> DepsResolver<'a, F>
where
    F: Fn(&'a Identifier) -> Option<&'a DepsConf>,
{
    pub fn init(roots: HashSet<&'a Identifier>, get_deps_for: F) -> Self {
        DepsResolver {
            roots,
            get_deps_for,
        }
    }

    pub fn try_resolve(self) -> Result<Vec<&'a Identifier>, Box<dyn Error>> {
        todo!("Dependecies resolving")
    }
}
