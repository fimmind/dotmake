use super::{Action, RuleActionsConf};
use crate::cli::OPTIONS;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
pub struct ShellScript(String);

impl Action for ShellScript {
    fn perform(&self, conf: &RuleActionsConf) -> Result<(), Box<dyn Error>> {
        let mut shell = Command::new(&conf.shell)
            .current_dir(OPTIONS.dotfiles_dir())
            .stdin(Stdio::piped())
            .spawn()?;
        shell.stdin.as_mut().unwrap().write(&self.0.as_bytes())?;
        let exit_status = shell.wait()?;
        if !exit_status.success() {
            Err(match exit_status.code() {
                Some(code) => format!("Shell process exited with status code: {}", code),
                None => format!("Shell process exited with error"),
            })?;
        }
        Ok(())
    }
}
