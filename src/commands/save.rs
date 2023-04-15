use crate::commands::{CLICommand, Save};
use crate::{header, lfs, RunSettings};

// https://stackoverflow.com/questions/2481338/committing-to-a-branch-thats-not-checked-out
impl CLICommand for Save {
    fn exec(&self, settings: RunSettings) {
        header!("Saving {:?}", self.path);

        // check for changes to locked files

        // if not in claim branch, make claim branch

        // commit work

        // push changes
    }
}
