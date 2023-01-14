use crate::commands::{CLICommand, Status};
use crate::{header, info, lfs};

impl CLICommand for Status {
    fn exec(&self) {
        let lfs_locks = lfs::locked_files();

        // current workspace info
        let curr_name = "";
        let curr_locks = [""];

        header!("Current Workspace");
        info!("todo");

        header!("All Locks");
        info!("todo");

        header!("Local Workspaces");
        info!("todo");
    }
}
