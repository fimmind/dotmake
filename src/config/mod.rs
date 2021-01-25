mod deserializers;
pub mod rule_actions;

use crate::cli::OPTIONS;
use crate::deps_graph::DepsConf;
use crate::identifier::Identifier;
use crate::os::{self, OSError};
use lazy_static::lazy_static;
use rule_actions::{RuleActions, RuleActionsConf, RuleActionsError};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

lazy_static! {
    pub static ref CONFIG: Config = Config::init().unwrap_or_else(exit_error_fn!());
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
    UndefinedRule(Identifier),

    #[error(transparent)]
    OSError(#[from] OSError),
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        Ok(Self::parse_from(&Self::path()?)?)
    }

    fn parse_from(path: &PathBuf) -> Result<Self, ConfigError> {
        Ok(serde_yaml::from_reader(os::open_file(path)?)?)
    }

    fn path() -> Result<PathBuf, ConfigError> {
        Ok(OPTIONS
            .dotfiles_dir()
            .join(format!("dotm-{}.yaml", OPTIONS.distro_id()?)))
    }

    pub fn get_rule<'a>(&'a self, ident: &'a Identifier) -> Result<Rule<'a>, ConfigError> {
        Ok(Rule {
            actions: self
                .rules
                .get(&ident)
                .ok_or_else(|| ConfigError::UndefinedRule(ident.clone()))?,
            actions_conf: &self.actions_conf,
            ident,
        })
    }
}

pub struct Rule<'a> {
    actions: &'a RuleActions,
    actions_conf: &'a RuleActionsConf,
    ident: &'a Identifier,
}

#[derive(Debug, Error)]
pub enum RuleError {
    #[error("Failed to perfrom `{rule}`: {err}")]
    FailedToPerform {
        #[source]
        err: RuleActionsError,
        rule: Identifier,
    },
}

impl<'a> Rule<'a> {
    pub fn ident(&self) -> &Identifier {
        self.ident
    }

    pub fn deps_conf(&self) -> &'a DepsConf<Identifier> {
        self.actions.deps_conf()
    }

    pub fn perform_nth(&self, n: usize) -> Result<(), RuleError> {
        Ok(self
            .actions
            .peform_nth(n, self.actions_conf)
            .map_err(|err| RuleError::FailedToPerform {
                rule: self.ident.clone(),
                err,
            })?)
    }

    pub fn perform(&self) -> Result<(), RuleError> {
        Ok(self
            .actions
            .perform(self.actions_conf)
            .map_err(|err| RuleError::FailedToPerform {
                rule: self.ident.clone(),
                err,
            })?)
    }
}
