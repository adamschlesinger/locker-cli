use crate::commands::{CLICommand, Release};
use crate::lfs;

impl CLICommand for Release {
    fn exec(&self) {
        match &self.path {
            Some(path) => {}
            None => {
                // get all locks for the workspace/branch
                let locks = ["", ""];

                for lock in locks {}
            }
        }

        // match lfs::unlock(self.path.as_str(), false) {
        //     Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
        //     Err(err) => {}
        // }
    }
}
