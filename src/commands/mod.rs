//! todo

mod checkout;
mod claim;
mod commit;
mod config;
mod r#return;
mod save;
mod status;

use clap::Args;

/// Common trait for all subcommands
pub trait CLICommand {
    fn exec(&self);
}

/// todo
#[derive(Args, Debug)]
pub struct Claim {
    /// File or directory to be claimed to the current or specified workspace
    path: String,

    /// The workspace to claim the specified files to.
    #[arg(short, long)]
    workspace: Option<String>,
}

/// todo
#[derive(Args, Debug)]
pub struct Config {}

/// todo
#[derive(Args, Debug)]
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

/// todo
#[derive(Args, Debug)]
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
#[derive(Args, Debug)]
pub struct Checkout {}

/// todo
#[derive(Args, Debug)]
pub struct Commit {}

/// Output the status of locker
#[derive(Args, Debug)]
pub struct Status {}
