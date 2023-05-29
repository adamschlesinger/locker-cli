//! Ask a question and get an answer.

use crate::{debug, error};
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{event, execute, queue, QueueableCommand};
use std::error::Error;
use std::io::{stdin, stdout, Write};

/// Ask a question and get an answer.
/// # Open ended:
/// ```rust
/// let choice = question!("What is your name?");
/// ```
///
/// # With a default:
/// ```rust
/// let choice = question!(
///     "What is the answer to life, the universe, and everything?",
///     "42"
/// );
/// ```
///
/// # With defined options:
/// ``` rust
/// question!("Would you like to continue?" {
///     "yes" => todo!(),
///     "no" => todo!()
/// });
/// ```
///
/// # With single character options:
/// ``` rust
/// question!("Would you like to continue?" {
///     'y' => todo!(),
///     'n' => todo!()
/// });
/// ```
#[macro_export]
macro_rules! question {
    ($msg:tt {
        $($opt:tt => $response:expr),+$(,)*
    }) => {
        let answer = $crate::question::Question::new($msg)
            $(.with_option($opt))+
            .ask();

        match answer.as_str() {
            $($opt => $response),*
            &_ => {
                $crate::error!("Invalid Input");
                std::process::exit(exitcode::USAGE);
            }
        }
    };
    ($msg:tt, $default:expr) => {{
        $crate::question::Question::new($msg)
            .with_default($default)
            .ask()
    }};
    ($msg:tt) => {{
        $crate::question::Question::new(format!($msg))
            .ask()
    }};
}

#[doc(hidden)]
pub fn __question(msg: String) -> String {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!(" {msg}\n ")),
        SetForegroundColor(Color::White),
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

/// Ask a question and get an answer.
pub struct Question {
    text: String,
    default: Option<String>,
    options: Vec<String>,
}

impl Question {
    /// todo
    pub fn new(text: &str) -> Question {
        Question {
            text: text.into(),
            default: None,
            options: vec![],
        }
    }

    /// todo
    pub fn with_default(&mut self, default: &str) -> &mut Question {
        self.default = Some(default.into());
        self
    }

    /// todo
    pub fn with_option(&mut self, option: &str) -> &mut Question {
        self.options.push(option.into());
        self
    }

    /// todo
    pub fn ask(&self) -> String {
        let _ = print_question(self);
        get_input()
    }
}

fn print_question(question: &Question) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.queue(SetForegroundColor(Color::Yellow))?;
    stdout.queue(Print(format!(" {}", &question.text)))?;

    if !question.options.is_empty() {
        let mut index = 1;
        for option in &question.options {
            stdout.queue(Print(format!("\n [{index}] {option}")))?;
            index += 1;
        }
    } else if let Some(default) = &question.default {
        stdout.queue(Print(format!(" Hit ENTER to use default ({default})")))?;
    }

    stdout.queue(Print("\n "))?;
    stdout.queue(SetForegroundColor(Color::White))?;
    stdout.flush()?;

    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    let result = stdin().read_line(&mut input);

    if let Err(e) = result {
        error!("{:?}", e);
        std::process::exit(exitcode::USAGE);
    }

    debug!("Input is {:?}", input);
    input.trim().to_string()
}

fn get_input_char(options: &[char]) -> Result<char, Box<dyn Error>> {
    enable_raw_mode()?;

    let event = loop {
        if let Event::Key(event) = event::read()? {
            break event;
        }
    };

    for opt in options {
        if event.code == KeyCode::Char(*opt) {
            disable_raw_mode()?;
            return Ok(*opt);
        }
    }

    disable_raw_mode()?;

    error!("Invalid Input");
    std::process::exit(exitcode::USAGE);
}
