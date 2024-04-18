use crate::commands::{CLICommand, Switch};
use crate::{header, RunConfig};

impl CLICommand for Switch {
    fn exec(self, run_config: &RunConfig) {
        header!("Switching to workspace");
    }
}
