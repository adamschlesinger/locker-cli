use clap::{ArgMatches, Error, FromArgMatches, Parser, Subcommand, Args};

mod lfs;
mod commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct LockerInterface {
    #[clap(subcommand)]
    command: LockerCommand,
}

#[derive(Subcommand, Debug)]
enum LockerCommand {
    /// todo
    Admin,

    /// Unlock the specified file for you to work on; prevents other claims until returned. By
    /// default also creates a new branch to work in
    Claim(commands::claim::Claim),

    /// Lock a file, generate a PR, copy link to PR to clipboard
    Return,

    Checkout,

    /// Generate a commit of changes for all currently claimed files (default)
    Commit
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

    match &cli.command {
        LockerCommand::Admin => {
            println!("todo")
        },
        LockerCommand::Claim(baz) => {
            let _output = lfs::lfs_cmd("push");

            match _output {
                Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
                Err(e) => println!("error")
            }
        },
        LockerCommand::Return => {
            println!("todo")
        },
        LockerCommand::Commit => {
            println!("todo")
        },
        _ => {
            todo!();
        }
    }
}
