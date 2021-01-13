use std::error::Error;
use crate::identifier::Identifier;

#[derive(Debug, Deserialize)]
pub struct ActionsConf {}

#[derive(Debug, Deserialize)]
pub struct RuleActions {}

impl RuleActions {
    pub fn perform_all(&self, conf: &ActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Rule actions")
    }

    pub fn perform(&self, actions_list: &[Identifier], conf: &ActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Rule actions")
    }
}
