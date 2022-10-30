use crate::commands::{ Claim, CLICommand };
use crate::lfs::*;

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