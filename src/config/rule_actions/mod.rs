mod shell_script;

use crate::deps_resolver::DepsConf;
use std::error::Error;
use thiserror::Error;

use shell_script::{ShellScript, TempDirShellScript};

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
type Links = (); // TODO
type Deps = (); // TODO
type PostDeps = (); // TODO

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

    /// Rule's post dependencies
    Post(PostDeps),
}

pub trait Action {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>>;
    fn get_deps_conf(&self) -> DepsConf {
        DepsConf::new()
    }
}

impl Action for () {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        todo!("Replace mock action with a real one")
    }
}

macro_rules! match_dyn_action {
    ($self: expr) => {
        match_dyn_action!($self; Pkgs, Shell, InTemp, Links, Deps, Post)
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

    fn get_deps_conf(&self) -> DepsConf {
        self.as_dyn_action().get_deps_conf()
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "Vec<Actions>")]
pub struct RuleActions {
    actions: Vec<Actions>,
    deps_conf: DepsConf,
}

impl From<Vec<Actions>> for RuleActions {
    fn from(actions: Vec<Actions>) -> Self {
        RuleActions {
            deps_conf: Self::get_deps_conf(&actions),
            actions,
        }
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
        self.actions
            .get(n - 1)
            .ok_or(RuleActionsError::IndexOutOfRange)?
            .perform(conf)?;
        Ok(())
    }

    pub fn deps_conf(&self) -> &DepsConf {
        &self.deps_conf
    }

    fn get_deps_conf(actions: &[Actions]) -> DepsConf {
        let mut deps_conf = DepsConf::new();
        for action in actions {
            deps_conf.merge(action.get_deps_conf())
        }
        deps_conf
    }
}
