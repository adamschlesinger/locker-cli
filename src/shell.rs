use crate::{error, info};
use std::process::{Command};

/// Helper for running shell commands with formatting
#[macro_export]
macro_rules! sh {
    ($arg:expr) => {
        $crate::shell::sh(format!($arg))
    };
}

/// Describes when a shell command has failed with a non-zero exit code
#[derive(Debug)]
pub struct Error {
    /// Exit code
    pub code:i32,

    /// Output from the command
    pub output:String
}

/// todo
pub type Result = core::result::Result<String, Error>;

/// Executes the cmd and returns when done
pub fn sh(cmd: String) -> Result {
    let cmd_result = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();

    let cmd_output = cmd_result.unwrap();
    let std_bytes = if cmd_output.status.success() {
        cmd_output.stdout
    } else {
        cmd_output.stderr
    };

    let code = match cmd_output.status.code() {
        Some(code) => code,
        None => 0
    };

    let output = String::from_utf8(std_bytes)
        .unwrap()
        .trim()
        .to_string();

    if !cmd_output.status.success() {
        error!("{}", output);
        return Err(Error { code, output });
    }

    return Ok(output);
}

/// Executes the cmd and returns a handle
/// Generally this is for command which may expect followup input
fn cmd_handle() {
    // todo
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
