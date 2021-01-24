use colored::*;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::fmt::{self, Display};
use std::str;
use crate::cli::OPTIONS;

// OUTPUT
// ------
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Info => write!(f, "{} ", "info:".bright_black()),
            MessageType::Warning => write!(f, "{} ", "warning:".yellow().bold()),
            MessageType::Error => write!(f, "{} ", "error:".red().bold()),
        }
    }
}

pub fn print_msg(msg: &str, msg_type: MessageType) {
    eprint!("{}", msg_type);

    let msg = textwrap::fill(msg, 80);
    if msg.lines().take(2).count() > 1 {
        eprint!("\n{}", textwrap::indent(&msg, "  "));
    } else {
        eprintln!("{}", msg);
    }
}

#[macro_export]
macro_rules! exit_error {
    ($($fmt_args:expr),+) => {{
        print_error!($($fmt_args),+);
        std::process::exit(1)
    }};
}

#[macro_export]
macro_rules! exit_error_fn {
    () => {
        |err| exit_error!("{}", err)
    };
    ($($fmt_args:expr),+) => {
        |err| exit_error!("{}: {}", format!($($fmt_args),+), err)
    };
}

#[macro_export]
macro_rules! print_error {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(&format!($($fmt_args),+), MessageType::Error);
    }};
}

#[macro_export]
macro_rules! print_warn {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(&format!($($fmt_args),+, MessageType::Warning));
    }};
}

#[macro_export]
macro_rules! print_info {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(&format!($($fmt_args),+), MessageType::Info);
    }};
}

// INPUT
// -----
pub fn confirm(prompt: &str, default: bool) -> bool {
    if OPTIONS.noconfirm() {
        default
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact()
            .unwrap_or_else(exit_error_fn!("Failed to perform input"))
    }
}
