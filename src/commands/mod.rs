//! todo

mod checkout;
mod claim;
mod commit;
mod init;
mod release;
mod save;
mod status;
mod sync;
mod new;
mod switch;

use crate::{LockerCommand, RunConfig};
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

/// Common trait for all subcommands
#[enum_dispatch(LockerCommand)]
pub trait CLICommand {
    /// todo
    fn exec(self, run_config: &RunConfig);
}

/// Setup for Locker
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Init {}

/// Claim ownership over a directory or file so that it may be worked on in the current
/// or specified workspace
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Claim {
    /// File or directory to be claimed to the current or specified workspace.
    path: String,

    /// Switch to the named workspace before processing claim. If no workspace of specified name
    /// exists then it will be created first.
    #[arg(short, long)]
    workspace: Option<String>,
}

/// todo
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Release {
    /// The file or directory to be released. If not specified
    /// then all currently locked files of the current branch will be released.
    #[arg(short, long)]
    path: Option<String>,

    /// Paths which have been modified may not be released. This option will discard
    /// any changes made.
    #[arg(short, long)]
    discard_changes: bool,
}

/// Backs up changes to the remote repository
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Save {
    /// The file or directory with multiple claimable files to be saved
    path: Option<String>,

    /// Save all currently claimed files
    #[arg(short, long)]
    all: bool,

    /// NYI Commit message to specify changes made. Default is TODO
    #[arg(short, long)]
    message: Option<String>,
}

/// todo
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Checkout {}

/// todo
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Commit {}

/// Output the status of locker
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Status {}

/// Updates the current branch with all changes from the main branch
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Sync {}

/// Create a new workspace
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct New {
    /// Name of the workspace to create
    name: String,
}

/// Switch to another workspace and claim all files owned by it
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Switch {}
