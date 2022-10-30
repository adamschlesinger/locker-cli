//! todo

mod config;
mod claim;
mod save;
mod checkout;
mod r#return;
mod commit;
mod status;

use clap::{Args, Command, Subcommand};
use crate::commands::config::show_config;
use crate::lfs::*;

/// Common trait for all subcommands
pub trait CLICommand {
    fn exec(&self);
}

/// todo
#[derive(Args, Debug)]
pub struct Claim {
    /// Individual file or directory of files to be claimed
    #[clap(short, long)]
    path: String,

    /// todo
    #[clap(short, long)]
    branch_name: Option<String>,
}

/// todo
#[derive(Args, Debug)]
pub struct Config {}

/// todo
#[derive(Args, Debug)]
pub struct Return {
    /// The file or directory with multiple claimable files to be returned
    #[clap(short, long)]
    path: String,

    /// Return all currently claimed files
    #[clap(short, long)]
    all: Option<bool>,
}

/// todo
#[derive(Args, Debug)]
pub struct Save {
    /// The file or directory with multiple claimable files to be saved
    #[clap(short, long)]
    path: Option<String>,

    /// Save all currently claimed files
    #[clap(short, long)]
    all: Option<bool>,

    /// todo
    #[clap(short, long)]
    message: String,
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