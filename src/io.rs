use crate::cli::OPTIONS;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::fmt::{self, Display};
use std::str;

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

pub fn print_msg(msg: impl Into<String>, msg_type: MessageType) {
    eprint!("{}", msg_type);

    let mut msg = msg.into();
    textwrap::fill_inplace(&mut msg, 80);
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
macro_rules! print_error {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Error);
    }};
}

#[macro_export]
macro_rules! print_warn {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Warning);
    }};
}

#[macro_export]
macro_rules! print_info {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Info);
    }};
}

// INPUT
// -----
pub fn confirm(prompt: &str, default: bool) -> bool {
    if OPTIONS.noconfirm() {
        default
    } else {
        let res = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact();
        match res {
            Err(err) => exit_error!("Failed to perform input: {}", err),
            Ok(val) => val,
        }
    }
}

#[macro_export]
macro_rules! confirm {
    ($($format_arg: expr),*; $default: expr) => {
        $crate::io::confirm(&format!($($format_arg),*), $default)
    };
}
