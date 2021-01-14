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

pub struct DepsResolver<F> {
    roots: Vec<Identifier>,
    get_deps_for: F,
}

impl<F> DepsResolver<F>
where
    F: Fn(&Identifier) -> Result<&DepsConf, Box<dyn Error>>,
{
    pub fn init(roots: Vec<Identifier>, get_deps_for: F) -> Self {
        DepsResolver {
            roots,
            get_deps_for,
        }
    }

    pub fn try_resolve(&self) -> Result<Vec<&Identifier>, Box<dyn Error>> {
        todo!("Dependecies resolving")
    }
}
