mod deps;
mod links;
mod shell_script;
mod pkgs;

use crate::identifier::Identifier;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;

use deps::Deps;
use links::Links;
use shell_script::{ShellScript, TempDirShellScript};
use pkgs::{PkgManagersConf, Pkgs};

#[derive(Debug, Error)]
pub enum RuleActionsError {
    #[error("Index out of range")]
    IndexOutOfRange,

    #[error(transparent)]
    Any(#[from] Box<dyn Error>),
}

#[derive(Debug, Deserialize)]
pub struct RuleActionsConf {
    shell: String,
    backup_dir: PathBuf,
    pkg_managers: PkgManagersConf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Actions {
    /// Install packages using preconfigured package managers
    Pkgs(Pkgs),

    /// Run shell script in the Dotfiles direcorty
    Shell(ShellScript),

    /// Run shell script in a temporal directory
    InTemp(TempDirShellScript),

    /// Create links for given files
    Links(Links),

    /// Rule's dependencies
    Deps(Deps),
}

pub trait Action {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>>;
    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        HashSet::new()
    }
}

macro_rules! match_dyn_action {
    ($self: expr) => {
        match_dyn_action!($self; Pkgs, Shell, InTemp, Links, Deps)
    };

    ($self: expr; $($action: ident),*) => {
        match $self {
            $(Actions::$action(ref a) => a as &dyn Action,)*
        }
    };
}

impl Actions {
    fn as_dyn_action(&self) -> &dyn Action {
        match_dyn_action!(self)
    }

    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        self.as_dyn_action().perform(conf)
    }

    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.as_dyn_action().get_deps(conf)
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RuleActions {
    actions: Vec<Actions>,
}

impl RuleActions {
    pub fn perform(&self, conf: &RuleActionsConf) -> Result<(), RuleActionsError> {
        for action in &self.actions {
            action.perform(conf)?;
        }
        Ok(())
    }

    pub fn peform_nth(&self, n: usize, conf: &RuleActionsConf) -> Result<(), RuleActionsError> {
        if n == 0 {
            return Err(RuleActionsError::IndexOutOfRange);
        }
        self.actions
            .get(n - 1)
            .ok_or(RuleActionsError::IndexOutOfRange)?
            .perform(conf)?;
        Ok(())
    }

    pub fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.actions.iter().map(|a| a.get_deps(conf)).flatten().collect()
    }
}
