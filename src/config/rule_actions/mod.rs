mod shell_script;

use crate::deps_resolver::DepsConf;
use std::error::Error;
use thiserror::Error;

use shell_script::ShellScript;

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
}

type Pkgs = (); // TODO
type InTemp = (); // TODO
type Links = (); // TODO
type Deps = (); // TODO
type PostDeps = (); // TODO

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Actions {
    /// Install packages using preconfigured package managers
    Pkgs(Pkgs),

    /// Run shell script in Dotfiles direcorty
    Shell(ShellScript),

    /// Create links for given files
    Links(Links),

    /// Rule's dependencies
    Deps(Deps),

    /// Rule's post dependencies
    Post(PostDeps),
}

pub trait Action {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>>;
}

impl Action for () {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Replace mock action with a real one")
    }
}

macro_rules! match_dyn_action {
    ($self: expr; $($action: ident),*) => {
        match $self {
            $($action(ref a) => Box::new(a as &dyn Action),)*
        }
    }
}

impl Actions {
    fn as_dyn_action(&self) -> Box<&dyn Action> {
        use Actions::*;
        match_dyn_action!(self; Pkgs, Shell, Links, Deps, Post)
    }

    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        self.as_dyn_action().perform(conf)
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "Vec<Actions>")]
pub struct RuleActions {
    actions: Vec<Actions>,
}

impl From<Vec<Actions>> for RuleActions {
    fn from(actions: Vec<Actions>) -> Self {
        RuleActions { actions }
    }
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
        Ok(self
            .actions
            .get(n - 1)
            .ok_or(RuleActionsError::IndexOutOfRange)?
            .perform(conf)?)
    }

    pub fn get_deps_conf(&self) -> &DepsConf {
        todo!("Rule actions deps conf")
    }
}
