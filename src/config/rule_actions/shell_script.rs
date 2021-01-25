use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use crate::os::run_shell_script;
use std::error::Error;
use tempdir::TempDir;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ShellScript {
    script: String,
}

impl Action for ShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        Ok(run_shell_script(
            &conf.shell,
            OPTIONS.dotfiles_dir(),
            &self.script,
        )?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TempDirShellScript {
    script: String,
}

impl Action for TempDirShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        let temp_dir = TempDir::new("dotmake")?;
        run_shell_script(&conf.shell, temp_dir.path(), &self.script)?;
        temp_dir.close()?;
        Ok(())
    }
}
