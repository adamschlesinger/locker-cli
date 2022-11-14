use std::io::stdout;
use crossterm::execute;
use crossterm::style::{Color, Print, SetForegroundColor};
/// Because all of the popular log libraries were dumb
use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        crate::logger::__log(crate::logger::LogLevel::Debug, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        crate::logger::__log(crate::logger::LogLevel::Info, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::__log(crate::logger::LogLevel::Warn, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        crate::logger::__log(crate::logger::LogLevel::Error, format!($($arg)*))
    };
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Panic,
    None,
}

/// Sets the LogLevel
pub fn init(level: LogLevel) {
    match LOG_LEVEL.set(level) {
        Ok(_) | Err(_) => {} // no op
    }
}

/// todo
#[doc(hidden)]
pub fn __log(level: LogLevel, msg: String) {
    match LOG_LEVEL.get() {
        None => {} // not yet init - just ignore
        Some(logger_level) => if level >= *logger_level {
            let result = execute!(
                stdout(),
                SetForegroundColor(color(&level)),
                Print(format!("{} - {msg}\n", token(&level)))
            );
        },
    }
}

static LOG_LEVEL: OnceCell<LogLevel> = OnceCell::new();

fn token(level:&LogLevel) -> &str {
    match level {
        LogLevel::Debug => "D",
        LogLevel::Info  => "I",
        LogLevel::Warn  => "W",
        LogLevel::Error => "X",
        LogLevel::Panic => "!",
        LogLevel::None  => " ",
    }
}

fn color(level:&LogLevel) -> Color {
    match level {
        LogLevel::Debug => Color::Grey,
        LogLevel::Info  => Color::White,
        LogLevel::Warn  => Color::Yellow,
        LogLevel::Error => Color::Red,
        LogLevel::Panic => Color::White,
        LogLevel::None  => Color::White,
    }
}