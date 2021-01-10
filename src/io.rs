macro_rules! msg_fmt {
    ($($fmt_args:expr),*) => {{
        format!("[{}] {}", env!("CARGO_PKG_NAME"), format!($($fmt_args),+))
    }};
}

// OUTPUT
// ------
macro_rules! print_msg {
    ($($fmt_args:expr),+) => {
        eprintln!("{}", msg_fmt!($($fmt_args),+));
    };
}

#[macro_export]
macro_rules! exit_error {
    ($($fmt_args:expr),+) => {{
        print_error!($($fmt_args),+);
        print_info!("exiting due to previous error");
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
    ($($fmt_args:expr),+) => {
        print_msg!("ERROR:   {}", format!($($fmt_args),+));
    };
}

#[macro_export]
macro_rules! print_warning {
    ($($fmt_args:expr),+) => {
        print_msg!("WARNING: {}", format!($($fmt_args),+));
    };
}

#[macro_export]
macro_rules! print_info {
    ($($fmt_args:expr),+) => {
        print_msg!("INFO:    {}", format!($($fmt_args),+));
    };
}

// INPUT
// -----
#[macro_export]
macro_rules! confirm {
    ($default:expr; $($fmt_arg:expr),+) => {
        if $crate::cli::OPTIONS.noconfirm() {
            $default
        } else {
            dialoguer::Confirm::new()
                .with_prompt(msg_fmt!($($fmt_arg),+))
                .default($default)
                .interact()
                .unwrap_or_else(exit_error_fn!("Failed to perform input"))
        }
    };
}
