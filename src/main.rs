use clap::{ArgMatches, Args, Error, FromArgMatches, Parser, Subcommand};
use trait_enum::*;
use crate::commands::*;

mod lfs;
mod commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct LockerInterface {
    /// Path to the directory of the git repo if not run from it
    #[clap(short, long)]
    directory: Option<String>,

    /// The command to run
    #[clap(subcommand)]
    command: LockerCommand,
}
/// todo - should this ignore "branches" and use a "workspace" concept?
/// todo - change branch command for switching which branch a claim is linked to,
/// todo - separate Add command or just also use Claim?
trait_enum! {
    /// todo
    #[derive(Subcommand, Debug)]
    pub enum LockerCommand: CLICommand {
        /// todo
        Config,

        /// Claim ownership over a directory or file so that it may be worked on
        Claim,

        /// Return a directory or file so it may be claimed by other users
        Return,

        /// Commit your work to a claim branch
        Save,

        /// Output the current status of locker
        Status,
    }
}

// todo - separate LockerCommand enum for different config?

// #[derive(Debug, Serialize, Deserialize)]
pub struct LockerConfig {
    /// todo
    return_branch: String,

    /// eg: claim/*, feature/*, etc. Supports options such as claim/{file_name}
    /// If not specified then users will be have to specify their own branch on Claim
    claim_branch_pattern: Option<String>,

    /// Do returned files require a review step before merging into return_branch?
    require_review: bool,
}

fn main() {
    // let cfg: LockerConfig = confy::load("lockerConfig")?;
    // println!("{:#?}", cfg);

    let cli = LockerInterface::parse();
    let deref: &dyn CLICommand = cli.command.deref();
    deref.exec();
}
