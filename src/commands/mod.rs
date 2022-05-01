//! todo

mod config;

use clap::{Args, Command, Subcommand };
use crate::commands::config::show_config;
use crate::lfs::*;


/// Common trait for all subcommands
/// so they can be executed
pub trait CLICommand {
    fn exec(&self);
}

/// todo
#[derive(Args, Debug)]
pub struct Claim {
    /// File or directory with multiple claimable files you want to claim
    #[clap(short, long)]
    path: String,

    /// todo
    #[clap(short, long)]
    branch_name: Option<String>
}

impl CLICommand for Claim {
    fn exec(&self) {

        let result = LFS::lock(self.path.as_str());
        match result {
            Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
            Err(err) => {}
        }
        // println!("the path is {:?}", self.path);
        //
        // let _output = LFS::cmd("push");//format!("lock {path}"));
        //
        // match _output {
        //     Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
        //     Err(e) => println!("error")
        // }
    }
}

/// todo
#[derive(Args, Debug)]
pub struct Config {

}

impl CLICommand for Config {
    fn exec(&self) {
        show_config();
    }
}

/// todo
#[derive(Args, Debug)]
pub struct Return {
    /// The file or directory with multiple claimable files to be returned
    #[clap(short, long)]
    path: String,

    /// Return all currently claimed files
    #[clap(short, long)]
    all: Option<bool>
}

impl CLICommand for Return {
    fn exec(&self) {

        let result = LFS::unlock(self.path.as_str(), false);
        match result {
            Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
            Err(err) => {}
        }
    }
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
    message: String
}

impl CLICommand for Save {
    fn exec(&self) {

        // if not in claim branch, make claim branch

        // commit work

        // push changes
    }
}

/// todo
#[derive(Args, Debug)]
pub struct Checkout {

}

impl CLICommand for Checkout {
    fn exec(&self) {
        todo!();
    }
}

/// todo
#[derive(Args, Debug)]
pub struct Commit {

}

impl CLICommand for Commit {
    fn exec(&self) {
        todo!();
    }
}