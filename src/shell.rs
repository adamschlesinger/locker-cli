use crate::{debug, error};
use std::process::Command;

/// Executes the cmd and returns when done
#[macro_export]
macro_rules! sh {
    ($arg:expr) => {
        $crate::shell::__sh(format!($arg))
    };
}

/// Describes when a shell command has failed with a non-zero exit code
#[derive(Debug)]
pub struct Error {
    /// Exit code
    pub code: i32,

    /// Output from the command
    pub output: String,
}

/// todo
pub type Result = core::result::Result<String, Error>;

#[doc(hidden)]
pub fn __sh(cmd: String) -> Result {
    debug!("sh -c {}", cmd);
    let cmd_result = Command::new("sh").arg("-c").arg(cmd).output();

    let cmd_output = cmd_result.unwrap();
    let std_bytes = if cmd_output.status.success() {
        cmd_output.stdout
    } else {
        cmd_output.stderr
    };

    let code = cmd_output.status.code().unwrap_or(0);
    let output = String::from_utf8(std_bytes).unwrap().trim().to_string();

    if !cmd_output.status.success() {
        error!("{}", output);
        return Err(Error { code, output });
    }

    Ok(output)
}
