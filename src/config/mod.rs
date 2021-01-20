pub mod deserializers;
pub mod rule_actions;

use crate::cli::OPTIONS;
use crate::deps_resolver::DepsConf;
use crate::identifier::Identifier;
use crate::os::{self, OSError};
use lazy_static::lazy_static;
use rule_actions::{RuleActionsConf, RuleActions};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;

lazy_static! {
    pub static ref CONFIG: Config = Config::init().unwrap_or_else(exit_error_fn!());
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    actions_conf: RuleActionsConf,

    #[serde(default)]
    rules: HashMap<Identifier, RuleActions>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to parse config: {0}")]
    ParsingError(#[from] serde_yaml::Error),

    #[error("Undefined rule: {0}")]
    UnknownRule(Identifier),

    #[error("Failed to perfrom `{rule}`: {err}")]
    FailedToPerform {
        #[source]
        err: Box<dyn Error>,
        rule: Identifier,
    },

    #[error(transparent)]
    OSError(#[from] OSError),
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        Ok(Self::parse(&Self::get_config_path()?)?)
    }

    fn parse(path: &PathBuf) -> Result<Self, ConfigError> {
        Ok(serde_yaml::from_reader(os::open_file(path)?)?)
    }

    fn get_rule_actions(&self, ident: &Identifier) -> Result<&RuleActions, ConfigError> {
        self.rules
            .get(ident)
            .ok_or_else(|| ConfigError::UnknownRule(ident.clone()))
    }

    pub fn get_rule_deps_conf(&self, ident: &Identifier) -> Result<&DepsConf, ConfigError> {
        Ok(self.get_rule_actions(ident)?.get_deps_conf())
    }

    pub fn perform_rule(&self, ident: &Identifier) -> Result<(), ConfigError> {
        self.get_rule_actions(ident)?
            .perform_all(&self.actions_conf)
            .map_err(|err| ConfigError::FailedToPerform {
                rule: ident.clone(),
                err,
            })
    }

    pub fn perform_rule_actions(
        &self,
        ident: &Identifier,
        actions_list: &[Identifier],
    ) -> Result<(), ConfigError> {
        self.get_rule_actions(ident)?
            .perform(actions_list, &self.actions_conf)
            .map_err(|err| ConfigError::FailedToPerform {
                rule: ident.clone(),
                err,
            })
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        Ok(OPTIONS
            .dotfiles_dir()
            .join(format!("dotm-{}.yaml", OPTIONS.distro_id()?)))
    }
}
