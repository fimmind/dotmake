mod parser;

use crate::actions::{ActionsConf, RuleActions};
use crate::deps_resolver::DepsConf;
use crate::identifier::Identifier;
use lazy_static::lazy_static;
use parser::parse_config;
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
    pub static ref CONFIG: Config = parse_config().unwrap_or_else(exit_error_fn!());
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    actions_conf: ActionsConf,

    #[serde(default)]
    rules: HashMap<Identifier, RuleBody>,
}

impl Config {
    fn try_get_rule(&self, ident: &Identifier) -> Result<&RuleBody, Box<dyn Error>> {
        self.rules
            .get(ident)
            .ok_or_else(|| format!("Unknown rule: {}", ident).into())
    }

    pub fn try_get_rule_deps_conf(&self, ident: &Identifier) -> Result<&DepsConf, Box<dyn Error>> {
        self.try_get_rule(ident).map(|rule| &rule.deps_conf)
    }

    pub fn perform_rule(&self, ident: &Identifier) -> Result<(), Box<dyn Error>> {
        self.try_get_rule(ident)?
            .actions
            .perform_all(&self.actions_conf)
    }

    pub fn perform_rule_actions(
        &self,
        ident: &Identifier,
        actions_list: &[Identifier],
    ) -> Result<(), Box<dyn Error>> {
        self.try_get_rule(ident)?
            .actions
            .perform(actions_list, &self.actions_conf)
    }
}

#[derive(Debug, Deserialize)]
pub struct RuleBody {
    #[serde(flatten)]
    deps_conf: DepsConf,

    #[serde(flatten)]
    actions: RuleActions,
}
