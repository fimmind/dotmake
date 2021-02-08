//! Rule actions that specify rule's dependencies and behavior

mod deps;
mod links;
mod pkgs;
mod shell_script;

use crate::identifier::Identifier;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;

use deps::Deps;
use links::Links;
use pkgs::{PkgManagersConf, Pkgs};
use shell_script::{ShellScript, TempDirShellScript};

/// Errors that can occure while performing or accessing rule's actions
#[derive(Debug, Error)]
pub enum RuleActionsError {
    #[error("Index out of range")]
    IndexOutOfRange,

    #[error("Failed to perform `{action}` action: {err}")]
    FailedToPerform { action: String, err: Box<dyn Error> },
}

/// Actions' configuration. Some actions can optionally use some of it's field
/// while getting dependencies list or being performed
#[derive(Debug, Deserialize)]
pub struct RuleActionsConf {
    shell: String,
    backup_dir: PathBuf,
    pkg_managers: PkgManagersConf,
}

/// A single action. Every rule consits of a list of such actions that are
/// performed independently from each other in the same order as the user
/// specifies them in configuration file
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RuleAction {
    /// Install packages using preconfigured package managers
    Pkgs(Pkgs),

    /// Run shell script in the Dotfiles direcorty
    Shell(ShellScript),

    /// Run shell script in a temporal directory
    InTemp(TempDirShellScript),

    /// Create links for given files
    Links(Links),

    /// State given rules as dependencies
    Deps(Deps),
}

/// General action trait
trait Action {
    /// Perform the action
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>>;

    /// Get action's dependencies, i.e. other rules that have to be installed
    /// before the rule that contains that action
    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        HashSet::new()
    }
}

impl RuleAction {
    /// Coerce enum variant `&dyn Action`
    fn as_dyn_action(&self) -> &dyn Action {
        macro_rules! match_dyn_action {
            ($self: expr; $($action: ident),*) => {{
                match $self {
                    $(RuleAction::$action(ref a) => a as &dyn Action),*
                }
            }};
        }
        match_dyn_action!(self; Pkgs, Shell, InTemp, Links, Deps)
    }

    /// Wrapper around [`Action::perform`]
    ///
    /// [`Action::perform`]: self::Action::perform
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), RuleActionsError> {
        self.as_dyn_action()
            .perform(conf)
            .map_err(|err| RuleActionsError::FailedToPerform {
                action: self.ident().to_owned(),
                err,
            })
    }

    /// Wrapper aroud [`Action::get_deps`]
    ///
    /// [`Action::get_deps`]: self::Action::get_deps
    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.as_dyn_action().get_deps(conf)
    }

    /// Get the action's identifier that can be understood by the user
    fn ident(&self) -> &'static str {
        // TODO: figure ouw hot get this names derived Deserialize implementation
        match self {
            RuleAction::Pkgs(_) => "pkgs",
            RuleAction::Shell(_) => "shell",
            RuleAction::InTemp(_) => "in_temp",
            RuleAction::Links(_) => "links",
            RuleAction::Deps(_) => "deps",
        }
    }
}

/// A list of actions that are parsed from a configuration file. They specify
/// rule's dependencies and everything that have to be done when the rule is
/// performed
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RuleActions {
    actions: Vec<RuleAction>,
}

impl RuleActions {
    /// Perform all the actions in the same order as they are specified in
    /// configuration file
    ///
    /// # Errors
    /// If any of the actions fails to perform, an occurred error is returned
    pub fn perform(&self, conf: &RuleActionsConf) -> Result<(), RuleActionsError> {
        for action in &self.actions {
            action.perform(conf)?;
        }
        Ok(())
    }

    /// Perform nth action (indexing from one)
    ///
    /// # Errors
    /// Returns an error if:
    /// - index is out of range (i.e. less then 1 or bigger then the total
    /// number of action)
    /// - the action fails to perform
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

    /// Collect dependencies of all the separate actions
    pub fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        self.actions
            .iter()
            .map(|a| a.get_deps(conf))
            .flatten()
            .collect()
    }
}
