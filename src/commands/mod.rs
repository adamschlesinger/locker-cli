//! todo

mod config;
mod claim;
mod save;
mod checkout;
mod r#return;
mod commit;
mod status;

use clap::Args;

/// Common trait for all subcommands
pub trait CLICommand {
    fn exec(&self);
}

/// todo
#[derive(Args, Debug)]
pub struct Claim {
    /// File or directory of files to be claimed
    path: String,

    /// NYI If specified then create a new branch when claiming
    #[arg(short, long)]
    branch_name: Option<String>,
}

/// todo
#[derive(Args, Debug)]
pub struct Config {}

/// todo
#[derive(Args, Debug)]
pub struct Return {
    /// The file or directory with multiple claimable files to be returned. If not specified
    /// then all currently locked files of the current branch will be returned.
    #[arg(short, long)]
    path: Option<String>,
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