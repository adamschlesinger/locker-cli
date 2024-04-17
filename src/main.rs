//! todo
//!

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::exit;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::defaults::*;
use crate::commands::*;
use crate::config::RunConfig;
use crate::terminal::logger;
use crate::terminal::logger::LogLevel;
use crate::terminal::question;

mod commands;
mod git;
mod utils;
mod terminal;
mod config;
mod defaults;

/// Commands available to the locker cli
#[enum_dispatch]
#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum LockerCommand {
    /// Setup for Locker
    Init,

    /// Create a new workspace
    New,

    /// Switch to another workspace and claim all files owned by it
    Switch,

    /// Claim ownership over a directory or file so that it may be worked on in the current
    /// or specified workspace
    Claim,

    /// Release a directory, file, or whole workspace so it may be claimed by other users
    Release,

    /// Backs up changes to the remote repository
    Save,

    /// Updates the current branch with all changes from the main branch
    Sync,

    /// Output the current status of locker
    Status,
}

/// Default CLI
#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct LockerInterface {
    /// Path to the directory of the git repo if not run from it
    #[arg(short, long)]
    directory: Option<String>,

    /// Log debug messages
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,

    /// Command to run
    #[command(subcommand)]
    command: LockerCommand,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = LockerInterface::parse();
    logger::init(cli.log_level);

    header!("Starting Locker");
    info!("Asset control in git for humans");

    debug!("Establishing paths");
    let repo_path = git::repo_absolute_path(cli.directory);
    let locker_path = format!("{repo_path}/{LOCKER_PATH}");
    let config_path = format!("{repo_path}/{CONFIG_PATH}");
    let workspaces_path = format!("{repo_path}/{WORKSPACES_PATH}");
    debug!("Locker main path => {locker_path}");

    debug!("Building run settings");
    let run_config = RunConfig {
        repo_path,
        locker_path,
        config_path,
        workspaces_path,
    };

    if let LockerCommand::Init(command) = cli.command {
        command.exec(&run_config);
    } else {
        let mut commands = vec![cli.command];

        debug!("Attempting to load configuration at {}", run_config.config_path);
        let locker_path = Path::new(&run_config.locker_path);
        let config_path = Path::new(&run_config.config_path);

        if !locker_path.exists() || !config_path.exists() {
            question!("Could not find configuration for Locker; Would you like to initialize this repo?" {
                "y" => commands.insert(0, LockerCommand::Init(Init {})),
                "n" => { exit(exitcode::USAGE);}
            });
        }

        for command in commands {
            debug!("Running {:?}", command);
            command.exec(&run_config);
        }
    }

    Ok(())
}
