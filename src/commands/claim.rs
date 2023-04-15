use crate::commands::{CLICommand, Claim};
use crate::{error, header, info, lfs, RunSettings};
use crossterm::style::{Color, Print, SetForegroundColor};
use std::process;

impl CLICommand for Claim {
    fn exec(&self, settings: RunSettings) {
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
                // are we in a workspace?

                // no - error
                error!("Not in a workspace and none specified");
                process::exit(exitcode::USAGE);
            }
        };

        let result = lfs::lock(self.path.as_str());
        match result {
            Ok(out) => {
                info!("Claimed {:?}", self.path)
            }
            Err(err) => todo!("an error"),
        }
    }
}
