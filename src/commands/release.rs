use crate::commands::{CLICommand, Release};
use crate::{header, RunConfig};

impl CLICommand for Release {
    fn exec(self, run_config: &RunConfig) {
        header!("Releasing {:?}", self.path);

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
