use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use crate::config::deserializers::List;
use crate::identifier::{Identifier, Identifiers};
use crate::os::run_shell_script;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Pkgs {
    pkgs: HashMap<Identifier, List<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PkgManagersConf {
    install_cmds: HashMap<Identifier, String>,
    deps: HashMap<Identifier, Identifiers>,
}

#[derive(Debug, Error)]
pub enum PkgsError {
    #[error("Undefined package manager `{0}`")]
    UndefinedPkgMngr(Identifier),
}

lazy_static! {
    static ref PKG_RE: Regex = Regex::new(r"%(%|pkg)").unwrap();
}
impl Pkgs {
    fn substitude_pkg<'a>(s: &'a str, pkg: &str) -> Cow<'a, str> {
        PKG_RE.replace_all(s, |caps: &Captures| match &caps[1] {
            "%" => "%",
            "pkg" => pkg,
            _ => unreachable!(),
        })
    }
}

impl Action for Pkgs {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn std::error::Error>> {
        for pkg_mngr in self.pkgs.keys() {
            conf.pkg_managers.get_cmd(pkg_mngr)?;
        }
        for (pkg_mngr, pkgs) in &self.pkgs {
            let pkg_mngr_cmd = conf.pkg_managers.get_cmd(pkg_mngr).unwrap();
            for pkg in pkgs.iter() {
                run_shell_script(
                    &conf.shell,
                    OPTIONS.dotfiles_dir(),
                    &Self::substitude_pkg(pkg_mngr_cmd, pkg),
                )?;
            }
        }
        Ok(())
    }

    fn get_deps(&self, conf: &RuleActionsConf) -> HashSet<Identifier> {
        let deps = self.pkgs.keys().map(|mgr| conf.pkg_managers.get_deps(mgr));
        deps.flatten().collect()
    }
}

impl PkgManagersConf {
    fn get_cmd(&self, mngr: &Identifier) -> Result<&String, PkgsError> {
        let undefined_pkg_error = || PkgsError::UndefinedPkgMngr(mngr.clone());
        self.install_cmds.get(mngr).ok_or_else(undefined_pkg_error)
    }

    fn get_deps(&self, mngr: &Identifier) -> impl Iterator<Item = Identifier> + '_ {
        self.deps.get(mngr).into_iter().flatten()
    }
}
