use crate::{debug, error, info};
use crossterm::execute;
use crossterm::style::{Color, Print, SetForegroundColor};
use std::io::{stdin, stdout};
use std::process::Command;

/// Executes the cmd and returns when done
#[macro_export]
macro_rules! sh {
    ($arg:expr) => {
        $crate::shell::sh(format!($arg))
    };
}

/// Ask a question and get an answer
///
/// ``` rust
/// question!("Would you like to continue?" {
///     "y" => todo!(),
///     "n" => todo!()
/// });
/// ```
/// Produces:
/// "Would you like to continue? y/n"
#[macro_export]
macro_rules! question {
    ($msg:tt {
        $($opt:tt => $response:expr),+
    }) => {
        let input = $crate::shell::__question(
            format!("{} ({})", $msg, stringify!($($opt)*))
        );

        match input.as_str() {
            $($opt => $response),*
            &_ => {
                $crate::error!("Invalid Input");
                std::process::exit(exitcode::USAGE);
            }
        }
    };
    ($msg:tt {
        $($opt:tt => $response:expr,)+
    }) => {
        let input = $crate::shell::__question(
            format!("{} ({})", $msg, stringify!($($opt)*))
        );

        match input.as_str() {
            $($opt => $response),*
            &_ => {
                $crate::error!("Invalid Input");
                std::process::exit(exitcode::USAGE);
            }
        }
    };
    ($msg:tt, $default:expr) => {{
        let input = $crate::shell::__question(
            format!("{} Hit ENTER to use default {:?}", $msg, $default)
        );
        if input.is_empty() {
            $default
        } else {
            input
        }
    }};
    ($msg:tt) => {{
        $crate::shell::__question(format!($msg))
    }};
}

// #[macro_export]
// #[doc(hidden)]
// macro_rules! __question_with_options {
//     ($msg:tt [$({$opt:tt, $response:expr}),+]) => {
//         // todo
//     };
// }

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
pub fn sh(cmd: String) -> Result {
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

#[doc(hidden)]
pub fn __question(msg: String) -> String {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!(" {msg}\n ")),
        SetForegroundColor(Color::DarkYellow),
    );

    let mut input = String::new();
    let result = stdin().read_line(&mut input);

    if let Err(e) = result {
        error!("{:?}", e);
        std::process::exit(exitcode::USAGE);
    }

    debug!("Input is {:?}", input);
    input.trim().to_string()
}
