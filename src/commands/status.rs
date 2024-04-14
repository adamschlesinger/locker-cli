use crate::commands::{CLICommand, Status};
use crate::{header, info, RunSettings};
use crate::git::lfs;

impl CLICommand for Status {
    fn exec(&self, settings: &RunSettings) {
        let lfs_locks = lfs::locked_files();

        // current workspace info
        let curr_name = "";
        let curr_locks = [""];

        header!("Current Workspace");
        info!("todo");

        header!("All Locks");
        info!("{:?}", lfs_locks.unwrap());

        // necessary to separate this info?
        header!("Local Workspaces");
        info!("todo");

        header!("Remote Workspaces");
        info!("todo");
    }
}
