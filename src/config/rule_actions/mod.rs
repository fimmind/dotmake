use crate::deps_resolver::DepsConf;
use std::error::Error;
use crate::identifier::Identifier;

#[derive(Debug, Deserialize)]
pub struct RuleActionsConf {}

#[derive(Debug, Deserialize)]
pub struct RuleActions {}

impl RuleActions {
    pub fn perform_all(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Rule actions")
    }

    pub fn perform(&self, actions_list: &[Identifier], conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Rule actions")
    }

    pub fn get_deps_conf(&self) -> &DepsConf {
        todo!("Rule actions")
    }
}
