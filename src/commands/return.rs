use crate::commands::{Return, CLICommand};
use crate::lfs;

struct QWE {
    path: Option<String>,
}

impl CLICommand for QWE {
    fn exec(&self) {}
}

impl CLICommand for Return {
    fn exec(&self) {

        // match self.path {
        //     None => {}
        //     Some(qwe) => {}
        // }

        // if let Some(qwe) = self.path {
        //
        // }

        // match lfs::unlock(self.path.as_str(), false) {
        //     Ok(out) => println!("{:?}", String::from_utf8(out.stdout)),
        //     Err(err) => {}
        // }
    }
}