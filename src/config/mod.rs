mod parser;

use crate::actions::{ActionsConf, RuleActions};
use crate::deps_resolver::DepsConf;
use crate::identifier::Identifier;
use lazy_static::lazy_static;
use parser::{parse_config, ParsingError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
use thiserror::Error;

use crate::cli::OPTIONS;

lazy_static! {
    pub static ref CONFIG: Config =
        parse_config(&get_config_path().unwrap_or_else(exit_error_fn!()))
            .unwrap_or_else(exit_error_fn!());
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    actions_conf: ActionsConf,

    #[serde(default)]
    rules: HashMap<Identifier, RuleBody>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] ParsingError),

    #[error("Unknown rule: {0}")]
    UnknownRule(Identifier),

    #[error("Failed to perfrom `{rule}`: {err}")]
    FailedToPerform {
        #[source]
        err: Box<dyn Error>,
        rule: Identifier,
    },

    #[error("Failed to read `{file}`: {err}")]
    FailedToRead {
        #[source]
        err: io::Error,
        file: PathBuf,
    },

    #[error("`/etc/os-release` does not contain ID field")]
    DistroIdNotSpecified,
}

impl Config {
    fn try_get_rule(&self, ident: &Identifier) -> Result<&RuleBody, ConfigError> {
        self.rules
            .get(ident)
            .ok_or_else(|| ConfigError::UnknownRule(ident.clone()))
    }

    pub fn try_get_rule_deps_conf(&self, ident: &Identifier) -> Result<&DepsConf, ConfigError> {
        self.try_get_rule(ident).map(|rule| &rule.deps_conf)
    }

    pub fn perform_rule(&self, ident: &Identifier) -> Result<(), ConfigError> {
        self.try_get_rule(ident)?
            .actions
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
        self.try_get_rule(ident)?
            .actions
            .perform(actions_list, &self.actions_conf)
            .map_err(|err| ConfigError::FailedToPerform {
                rule: ident.clone(),
                err,
            })
    }
}

#[derive(Debug, Deserialize)]
pub struct RuleBody {
    #[serde(flatten)]
    deps_conf: DepsConf,

    #[serde(flatten)]
    actions: RuleActions,
}

fn get_config_path() -> Result<PathBuf, ConfigError> {
    Ok(OPTIONS
        .dotfiles_dir()
        .join(format!("dotm-{}.yaml", get_distro_id()?)))
}

fn get_distro_id() -> Result<String, ConfigError> {
    match OPTIONS.distro_id() {
        Some(id) => Ok(id.to_string()),
        None => get_default_distro_id(),
    }
}

fn get_default_distro_id() -> Result<String, ConfigError> {
    let os_release = "/etc/os-release";
    let failed_to_read = |err| ConfigError::FailedToRead {
        file: os_release.into(),
        err,
    };

    let reader = BufReader::new(File::open(os_release).map_err(failed_to_read)?);
    for line in reader.lines() {
        let line = line.map_err(failed_to_read)?;
        if line.starts_with("ID=") {
            return Ok(line[3..].to_string());
        }
    }
    Err(ConfigError::DistroIdNotSpecified)
}
