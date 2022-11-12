use std::process::{Command};

/// todo
pub type ShellResult = core::result::Result<String, ShellError>;

/// todo - the message
pub struct ShellError {}

/// Executes the cmd and returns when done
pub fn sh(cmd: String) -> ShellResult {
    let cmd_result = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();

    let str_result = match cmd_result {
        Ok(cmd_output) => String::from_utf8(cmd_output.stdout),
        Err(cmd_err) => return Err(ShellError {})
    };

    return match str_result {
        Ok(output_str) => Ok(output_str),
        Err(utf8_err) => Err(ShellError {})
    };
}

/// Executes the cmd and returns a handle
/// Generally this is for command which may expect followup input
fn cmd_handle() {
    // todo
}

/// Helper for running shell commands with formatting
#[macro_export]
macro_rules! sh {
    ($arg:expr) => {
        $crate::shell::sh(format!($arg))
    }
}

// Build simple methods to contain a shell command
#[macro_export]
macro_rules! sh_fn {
    ($funcname:tt, $command:tt, $($varname:tt: $typename:ty),*) => {
        pub fn $funcname($($varname: $typename),*) -> ShellResult {
            return sh!($command);
        }
    };
}