use crate::commands::{ Return, CLICommand };
use crate::lfs::*;

impl CLICommand for Return {
    fn exec(&self) {

        let result = LFS::unlock(self.path.as_str(), false);
        match result {
            Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
            Err(err) => {}
        }
    }
}