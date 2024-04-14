use std::path::Path;
use std::process;

use crate::{debug, error, header, info, RunSettings};
use crate::commands::{Claim, CLICommand};
use crate::git::lfs;

impl CLICommand for Claim {
    fn exec(&self, settings: &RunSettings) {
        header!("Claiming {:?}", self.path);

        match &self.workspace {
            Some(workspace) => {
                // is this the workspace we are in now?
                //   yes - break
                //   no - does this workspace already exist?
                //     no - make it
                // switch to workspace
            }
            None => {
                debug!("No workspace specified. Checking if one is currently active.");
                // todo

                // no - error
                error!("Not in a workspace and none specified");
                process::exit(exitcode::USAGE);
            }
        };

        // does path exist?
        if Path::new(&self.path).exists() {
            let result = lfs::lock(self.path.as_str());
            match result {
                Ok(out) => {
                    info!("Claimed {:?}", self.path)
                }
                Err(err) => todo!("an error"),
            }
        } else {
            debug!("Checking if {} is a workspace", self.path);
        }
    }
}
