extern crate core;

use crate::commands::*;
use crate::logger::LogLevel;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;
use trait_enum::*;

mod commands;
mod git;
mod lfs;
mod logger;
mod shell;

// todo - should this ignore "branches" and use a "workspace" concept?
// todo - change branch command for switching which branch a claim is linked to,
// todo - separate Add command or just also use Claim?
trait_enum! {
    /// Commands available to the locker cli
    #[derive(Subcommand, Debug)]
    pub enum LockerCommand: CLICommand {
        /// Setup for Locker
        Init,

        /// Claim ownership over a directory or file so that it may be worked on in the current
        /// or specified workspace
        Claim,

        /// Release a directory, file, or whole workspace so it may be claimed by other workspaces
        Release,

        /// Backs up changes to the remote repository
        Save,

        /// Output the current status of locker
        Status
    }
}

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

/// Global settings passed to the executed command for this run
pub struct RunSettings<'a> {
    /// todo
    repo_path: &'a Path,

    /// todo
    locker_path: &'a Path,

    /// todo
    config_path: &'a Path,

    /// todo
    workspaces_path: &'a Path,
}

/// todo
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// todo
    name: String,

    /// All paths currently owned by this workspace
    paths: Vec<String>,
}

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

const LOCKER_PATH: &str = ".locker";
const CONFIG_PATH: &str = ".locker/config";
const WORKSPACES_PATH: &str = ".locker/workspaces";

/// Default CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct LockerInterface {
    /// Path to the directory of the git repo if not run from it
    #[arg(short, long)]
    directory: Option<String>,

    /// Log debug messages
    #[arg(short, long)]
    verbose: bool,

    /// Command to run
    #[command(subcommand)]
    command: LockerCommand,
}

/// CLI if the detected user is an admin
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct LockerInterfaceAdmin {
    /// Path to the directory of the git repo if not run from it
    #[arg(short, long)]
    directory: Option<String>,

    /// Log all messages
    #[arg(short, long)]
    verbose: bool,

    /// The command to run
    #[command(subcommand)]
    command: LockerCommand,
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

    debug!("Establishing paths");
    let repo_path = git::repo_absolute_path(cli.directory);
    let locker_path = format!("{repo_path}/{LOCKER_PATH}");
    let config_path = format!("{repo_path}/{CONFIG_PATH}");
    let workspaces_path = format!("{repo_path}/{WORKSPACES_PATH}");
    debug!("Repo base path   {repo_path}");
    debug!("Locker main path {locker_path}");
    debug!("Config path      {config_path}");
    debug!("Workspaces path  {workspaces_path}");

    debug!("Attempting to load configuration at {config_path}");
    let repo_path = Path::new(&repo_path);
    let locker_path = Path::new(&locker_path);
    let config_path = Path::new(&config_path);
    let workspaces_path = Path::new(&workspaces_path);

    // if !locker_path.exists() || !config_path.exists() {
    //     error!("Could not find configuration for Locker; please run `locker init`");
    //     std::process::exit(exitcode::USAGE);
    // }

    debug!("Building run settings");
    let settings = RunSettings {
        repo_path,
        locker_path,
        config_path,
        workspaces_path,
    };

    // run the command
    debug!("{:?}", cli.command);

    let deref: &dyn CLICommand = cli.command.deref();
    deref.exec(settings);

    Ok(())
}
