//! Various helper functions and macros for simple, pretty and unified I/O

use crate::cli::OPTIONS;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::str;

/// Supported massage types
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

impl MessageType {
    /// Get message prefix for a given message type
    fn prefix(&self) -> ColoredString {
        match self {
            MessageType::Info => "info:".bright_black(),
            MessageType::Warning => "warning:".yellow().bold(),
            MessageType::Error => "error:".red().bold(),
        }
    }
}

/// Print a message of a given type to stderr
pub fn print_msg(msg: impl Into<String>, msg_type: MessageType) {
    eprint!("{} ", msg_type.prefix());

    let mut msg = msg.into();
    textwrap::fill_inplace(&mut msg, 80);
    if msg.lines().take(2).count() > 1 {
        eprint!("\n{}", textwrap::indent(&msg, "  "));
    } else {
        eprintln!("{}", msg);
    }
}

/// Print error message to stderr and exit with status code 1
#[macro_export]
macro_rules! exit_error {
    ($($fmt_args:expr),+) => {{
        print_error!($($fmt_args),+);
        std::process::exit(1)
    }};
}

/// Print error message to stderr
#[macro_export]
macro_rules! print_error {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Error);
    }};
}

/// Print warning message to stderr
#[macro_export]
macro_rules! print_warn {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Warning);
    }};
}

/// Waring info message to stderr
#[macro_export]
macro_rules! print_info {
    ($($fmt_args:expr),+) => {{
        use $crate::io::*;
        print_msg(format!($($fmt_args),+), MessageType::Info);
    }};
}

/// Confirmation prompt rendered at stderr
///
/// If noconfirm option is set by the user, `default` is returned without of any
/// prompt being displayed
pub fn confirm(prompt: &str, default: bool) -> bool {
    if OPTIONS.noconfirm() {
        default
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact()
            .unwrap()
    }
}

/// Confirmation prompt rendered at stderr
///
/// See [`confirm`] function for more details
///
/// [`confirm`]: io/fn.confirm.html
#[macro_export]
macro_rules! confirm {
    ($($format_arg: expr),*; $default: expr) => {
        $crate::io::confirm(&format!($($format_arg),*), $default)
    };
}
