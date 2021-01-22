use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use std::error::Error;
use crate::os::run_shell_script;

#[derive(Debug, Deserialize)]
pub struct ShellScript(String);

impl Action for ShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(run_shell_script(&conf.shell, OPTIONS.dotfiles_dir(), &self.0)?)
    }
}
