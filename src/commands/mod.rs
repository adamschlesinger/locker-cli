//! todo

use clap::{Args, Command, Subcommand };
use crate::lfs;

/// Common trait for all subcommands so they can
/// be executed
pub trait CLICommand {
    fn exec(&self);
}

#[derive(Args, Debug)]
pub struct Claim {
    /// Which file or path with multiple claimable files you want to claim
    #[clap(short, long)]
    pub path: Option<String>,
}

impl CLICommand for Claim {
    fn exec(&self) {
        println!("the path is {:?}", self.path);

        let _output = lfs::lfs_cmd("push");//format!("lock {path}"));

        match _output {
            Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
            Err(e) => println!("error")
        }
    }
}

#[derive(Args, Debug)]
pub struct Admin {

}


impl CLICommand for Admin {
    fn exec(&self) {

        println!("yay admins!");
    }
}