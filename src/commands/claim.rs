use crate::commands::{ Claim, CLICommand };
use crate::lfs;

impl CLICommand for Claim {
    fn exec(&self) {
        let result = lfs::lock(self.path.as_str());
        match result {
            Ok(out) => println!("{:?}", out),
            Err(err) => todo!("put an error here")
        }
    }
}