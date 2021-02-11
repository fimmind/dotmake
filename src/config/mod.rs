//! Config abstraction that handles parsing and interactions with config file

mod deps_graph;
mod deserializers;
mod rule_actions;

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

/// Config file abstraction
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "conf")]
    actions_conf: RuleActionsConf,

    #[serde(default)]
    rules: HashMap<Identifier, RuleActions>,
}

/// Various errors that can occure while interacting with configuration file
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
    /// Find and parse configuration file
    pub fn init() -> Result<Self, ConfigError> {
        Ok(Self::parse(&Self::base_path()?)?)
    }

    /// Get path (without an extension) where config file is expected to be
    /// found
    fn base_path() -> Result<PathBuf, ConfigError> {
        let dot_dir = OPTIONS.dotfiles_dir();
        let distro_id = OPTIONS.distro_id()?;
        Ok(dot_dir.join(format!("dotm-{}", distro_id)))
    }

    /// Parse configuration file with a given base name. This function goes
    /// through the list of abailable config formats end for each of them tests
    /// if there exists config file with a relevant extension. If one exists,
    /// result of parsing is returned
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

    /// Parse yaml file
    fn parse_yaml(file: &File) -> Result<Self, ConfigError> {
        Ok(serde_yaml::from_reader(file)?)
    }

    /// Parse json file
    fn parse_json(file: &File) -> Result<Self, ConfigError> {
        Ok(serde_json::from_reader(file)?)
    }

    /// Obtain rule with a given identifier, if one exists
    pub fn get_rule<'a>(&'a self, ident: &'a Identifier) -> Option<Rule<'a>> {
        self.rules.get(ident).map(|actions| Rule {
            actions_conf: &self.actions_conf,
            actions,
            ident,
        })
    }

    /// Obtain rule with a given identifier or return an error if one does not
    /// exist
    pub fn try_get_rule<'a>(&'a self, ident: &'a Identifier) -> Result<Rule<'a>, ConfigError> {
        self.get_rule(ident)
            .ok_or_else(|| ConfigError::UndefinedRule(ident.clone()))
    }

    /// Build rules' depnedencies graph
    ///
    /// It's graranteed that all the nodes of resulting graph refer to existing
    /// rules, i.e. you can safely unwrap `Config::get_rule` called on any the
    /// nodes
    pub fn get_deps_graph(&self) -> Result<DepsGraph<Identifier>, ConfigError> {
        let mut graph = HashMap::<_, HashSet<_>>::with_capacity(self.rules.len());
        for ident in self.rules.keys() {
            let deps = self.get_rule(ident).unwrap().get_deps();
            for dep in deps.iter() {
                self.try_get_rule(dep)?; // TODO: test that all identifiers are valid right after deserialization
            }
            graph.insert(ident.clone(), deps);
        }
        Ok(graph.into())
    }
}

/// A structure representing a single rule, that borrows actions configuration
/// from [`Config`]
pub struct Rule<'a> {
    actions: &'a RuleActions,
    actions_conf: &'a RuleActionsConf,
    ident: &'a Identifier,
}

/// Errors that can occure when interacting with [`Rule`]
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
    /// Get rule's dependencies
    pub fn get_deps(&self) -> HashSet<Identifier> {
        self.actions.get_deps(&self.actions_conf)
    }

    /// Perform nth (indexing from 1) action of the rule
    ///
    /// This is a wrapper aroud [`RuleActions::perform_nth`]
    pub fn perform_nth(&self, n: usize) -> Result<(), RuleError> {
        Ok(self
            .actions
            .perform_nth(n, self.actions_conf)
            .map_err(|err| RuleError::FailedToPerform {
                rule: self.ident.clone(),
                err,
            })?)
    }

    /// Perfrom all the actions of the rule
    ///
    /// This is a wrapper aroud [`RuleActions::perform`]
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
