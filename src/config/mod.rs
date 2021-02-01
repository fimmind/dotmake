mod deps_graph;
mod deserializers;
pub mod rule_actions;

use crate::cli::OPTIONS;
use crate::identifier::Identifier;
use crate::os::{self, OSError};
use deps_graph::DepsGraph;
use maplit::hashmap;
use rule_actions::{RuleActions, RuleActionsConf, RuleActionsError};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "conf")]
    actions_conf: RuleActionsConf,

    #[serde(default)]
    rules: HashMap<Identifier, RuleActions>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to parse config: {0}")]
    ParsingError(Box<dyn Error>),

    #[error("Config not found")]
    ConfigNotFound,

    #[error("Undefined rule: {0}")]
    UndefinedRule(Identifier),

    #[error(transparent)]
    OSError(#[from] OSError),
}

macro_rules! parser_errors {
    ($($err_type: ty),*$(,)?) => {
        $(impl From<$err_type> for ConfigError {
            fn from(err: $err_type) -> Self {
                ConfigError::ParsingError(Box::new(err).into())
            }
        })*
    };
}
parser_errors! {
    serde_yaml::Error,
    serde_json::Error,
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        Ok(Self::parse(&Self::base_path()?)?)
    }

    fn base_path() -> Result<PathBuf, ConfigError> {
        Ok(OPTIONS
            .dotfiles_dir()
            .join(format!("dotm-{}", OPTIONS.distro_id()?)))
    }

    fn parse(base_path: &Path) -> Result<Self, ConfigError> {
        type Parser = &'static dyn Fn(&File) -> Result<Config, ConfigError>;
        let parsers = hashmap! {
            "yaml" => &Self::parse_yaml as Parser,
            "json" => &Self::parse_json as Parser,
        };

        for (ext, parse) in parsers {
            let path = base_path.with_extension(ext);
            if path.exists() {
                return parse(&os::open_file(&path)?);
            }
        }
        Err(ConfigError::ConfigNotFound)
    }

    fn parse_yaml(file: &File) -> Result<Self, ConfigError> {
        Ok(serde_yaml::from_reader(file)?)
    }

    fn parse_json(file: &File) -> Result<Self, ConfigError> {
        Ok(serde_json::from_reader(file)?)
    }

    pub fn get_rule<'a>(&'a self, ident: &'a Identifier) -> Option<Rule<'a>> {
        self.rules.get(ident).map(|actions| Rule {
            actions_conf: &self.actions_conf,
            actions,
            ident,
        })
    }

    pub fn try_get_rule<'a>(&'a self, ident: &'a Identifier) -> Result<Rule<'a>, ConfigError> {
        self.get_rule(ident)
            .ok_or_else(|| ConfigError::UndefinedRule(ident.clone()))
    }

    pub fn get_deps_graph(&self) -> Result<DepsGraph<Identifier>, ConfigError> {
        let mut graph = HashMap::<_, HashSet<_>>::new();
        for ident in self.rules.keys() {
            let deps = self.get_rule(ident).unwrap().get_deps();
            for dep in deps.iter() {
                self.try_get_rule(dep)?; // TODO: test that all identifiers are valid right after deserialization
            }
            graph.entry(ident.clone()).or_default().extend(deps);
        }
        Ok(graph.into())
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

    pub fn get_deps(&self) -> HashSet<Identifier> {
        self.actions.get_deps(&self.actions_conf)
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
