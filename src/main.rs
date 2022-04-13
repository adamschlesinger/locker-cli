use clap::{ArgMatches, Args, Error, FromArgMatches, Parser, Subcommand};
use trait_enum::trait_enum;
use crate::commands::{ CLICommand, Claim, Admin };

mod lfs;
mod commands;

use std::ops::Deref;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct LockerInterface {
    #[clap(subcommand)]
    command: LockerCommand,
}

#[derive(Subcommand, Debug)]
enum LockerCommandDEPRECATED {
    /// todo
    Admin(commands::Admin),

    /// Unlock the specified file for you to work on; prevents other claims until returned. By
    /// default also creates a new branch to work in
    Claim(commands::Claim),

    /// Lock a file, generate a PR, copy link to PR to clipboard
    Return,

    /// todo
    Checkout,

    /// Generate a commit of changes for all currently claimed files (default)
    Commit
}

trait_enum!{
    #[derive(Subcommand, Debug)]
    pub enum LockerCommand: CLICommand {
        /// todo
        Admin,

        /// Unlock the specified file for you to work on; prevents other claims until returned. By
        /// default also creates a new branch to work in
        Claim
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LockerConfig {
//     name: String,
//     bar: bool,
//     foo: i64,
// }

fn main() {
    // let cfg: LockerConfig = confy::load("lockerConfig")?;
    // println!("{:#?}", cfg);

    let cli = LockerInterface::parse();
    println!("{:?}", cli);

    if let cmd = cli.command {
        cmd.exec();
    }
}
