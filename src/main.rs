extern crate core;

use crate::commands::*;
use crate::logger::LogLevel;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use trait_enum::*;

mod commands;
mod git;
mod lfs;
mod logger;
mod shell;

/// Path of locker's configs and workspace information
const LOCKER_PATH: &str = ".locker";
const WORKSPACES_PATH: &str = ".locker/workspaces";

/// Default CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct LockerInterface {
    /// NYI Path to the directory of the git repo if not run from it
    #[arg(short, long)]
    directory: Option<String>,

    /// NYI Log all messages
    #[arg(short, long)]
    verbose: bool,

    /// The command to run
    #[command(subcommand)]
    command: LockerCommand,
}

/// CLI if the detected user is an admin
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct LockerInterfaceAdmin {
    /// NYI Path to the directory of the git repo if not run from it
    #[arg(short, long)]
    directory: Option<String>,

    /// NYI Log all messages
    #[arg(short, long)]
    verbose: bool,

    /// NYI Force any command to resolve with expected behavior
    #[arg(short, long)]
    force: bool,

    /// The command to run
    #[command(subcommand)]
    command: LockerCommand,
}

// todo - should this ignore "branches" and use a "workspace" concept?
// todo - change branch command for switching which branch a claim is linked to,
// todo - separate Add command or just also use Claim?
trait_enum! {
    /// Commands available to the locker cli
    #[derive(Subcommand, Debug)]
    pub enum LockerCommand: CLICommand {
        /// todo
        Config,

        /// Claim ownership over a directory or file so that it may be worked on in the current
        /// or specified workspace
        Claim,

        /// Release an unchanged directory, file, or whole workspace so it may be claimed by other workspaces
        Release,

        /// Backs up changes to the remote repository
        Save,

        /// Output the current status of locker
        Status
    }
}

// todo - separate LockerCommand enum for different config?

/// Primary config for this repo's locker
#[derive(Debug, Serialize, Deserialize)]
pub struct LockerConfig {
    /// todo
    return_branch: String,

    /// eg: claim/*, feature/*, etc. Supports options such as claim/{file_name}
    /// If not specified then users will be have to specify their own branch on Claim
    claim_branch_pattern: Option<String>,

    /// Do returned files require a review step before merging into return_branch?
    require_review: bool,
}

/// todo
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {}

// todo - should workspace info be local only or remote?
// What do we gain by it being remote?
// - Ability to see the status of other workspaces.
// - Can we do it in a hidden way with some weird ass lfs commands?

/// Local file describing the created workspaces. Removed
/// when the workspace is submitted for review.
#[derive(Debug, Serialize, Deserialize)]
pub struct LockerWorkspace {
    /// The paths currently claimed to this workspace
    locked_paths: Vec<String>,
}

impl Default for LockerConfig {
    fn default() -> Self {
        LockerConfig {
            return_branch: git::origin_default(),
            claim_branch_pattern: None,
            require_review: false,
        }
    }
}

fn main() -> std::io::Result<()> {
    let cli = LockerInterface::parse();

    // setup logging
    logger::init(if cli.verbose {
        LogLevel::Debug
    } else {
        LogLevel::Info
    });

    header!("Starting Locker 🔐");

    let repo_path = git::repo_absolute_path(cli.directory);
    debug!("Absolute path to repo is {repo_path}");

    let locker_path = format!("{repo_path}/{LOCKER_PATH}");
    let config_path = format!("{locker_path}/config");
    debug!("Loading configuration at {config_path}");

    let locker_path = Path::new(&locker_path);
    let config_path = Path::new(&config_path);

    if !locker_path.exists() {
        debug!("Creating new path for configuration files");
        fs::create_dir_all(locker_path)?;
    }

    if !config_path.exists() {
        question!("Could not find a configuration for Locker. Would you like to set one up?" {
            "y" => {},
            "n" => {
                info!("Exiting...");
                return Ok(());
            }
        });

        let branch = question!("Specify branch for returned files.", git::origin_default());

        let config = LockerConfig {
            return_branch: branch,
            claim_branch_pattern: None,
            require_review: false,
        };

        debug!("Creating new config file");
        let cfg_str = toml::to_string(&LockerConfig::default()).unwrap();

        let mut file = File::create(config_path)?;
        file.write_all(cfg_str.as_bytes())?;
    }

    // run the command
    let deref: &dyn CLICommand = cli.command.deref();
    deref.exec();

    Ok(())
}
