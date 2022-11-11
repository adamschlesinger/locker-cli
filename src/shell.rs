use std::process::{Command};

/// todo
pub type ShellResult = core::result::Result<String, ShellError>;

/// todo - the message
pub struct ShellError {}

/// Executes the cmd and returns when done
pub fn cmd(arg: String) -> Result<String, ShellError> {
    let cmd_result = Command::new("sh")
        .arg("-c")
        .arg(arg)
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

/// Helper for running shell commands
#[macro_export]
macro_rules! sh {
    ($arg:expr) => {
        $crate::shell::cmd(format!($arg))
    }
}

// todo - this
macro_rules! build_cmd {
    ($funcname:tt, $command:tt, $($varname:tt: $typename:ty),*) => {
        pub fn $funcname($($varname: $typename),*) -> io::Result<Output> {
            return cmd(format!($command));
        }
    };
}

// build_fn!(qwe, "todo {path}", path: &str, force: bool);