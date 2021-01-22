use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use std::error::Error;
use crate::os::run_shell_script;
use tempdir::TempDir;

#[derive(Debug, Deserialize)]
pub struct ShellScript(String);

impl Action for ShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(run_shell_script(&conf.shell, OPTIONS.dotfiles_dir(), &self.0)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct TempDirShellScript(String);

impl Action for TempDirShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        let temp_dir = TempDir::new("dotmake")?;
        run_shell_script(&conf.shell, temp_dir.path(), &self.0)?;
        temp_dir.close()?;
        Ok(())
    }
}
