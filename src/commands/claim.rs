use std::path::Path;

use crate::{debug, header, info, RunConfig};
use crate::commands::{Claim, CLICommand};
use crate::config::WorkspaceConfig;
use crate::git::lfs;

impl CLICommand for Claim {
    fn exec(self, run_config: &RunConfig) {
        header!("Claiming {:?}", self.path);

        // let workspace = self.workspace
        //     .map_or_else(|| run_config.current_workspace, |ws_name| {
        //         Some(WorkspaceConfig::load(&ws_name)
        //             .unwrap_or(WorkspaceConfig::new(&ws_name)))
        //     })
        //     .expect("Not in a workspace and none specified!");

        // todo - make sure we have permissions to act on the workspace

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
