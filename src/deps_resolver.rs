use crate::config::Identifier;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DepsConf;

pub struct DepsResolver {}

impl DepsResolver {
    pub fn init(
        roots: Vec<Identifier>,
        get_deps_for: impl Fn(&Identifier) -> Result<&DepsConf, Box<dyn Error>>,
    ) -> Self {
        todo!("Dependecies resolving")
    }

    pub fn try_resolve(&self) -> Result<Vec<&Identifier>, Box<dyn Error>> {
        todo!("Dependecies resolving")
    }
}
