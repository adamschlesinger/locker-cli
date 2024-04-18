use crate::commands::{CLICommand, Sync};
use crate::{header, RunConfig};

impl CLICommand for Sync {
    fn exec(self, run_config: &RunConfig) {
        header!("Syncing");
    }
}
