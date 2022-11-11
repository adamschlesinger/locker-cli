use crate::sh;
use crate::shell::{ShellResult};

/// Name of the default branch
/// todo git rev-parse --abbrev-ref origin/HEAD
/// https://stackoverflow.com/questions/28666357/git-how-to-get-default-branch
pub fn origin_default() -> ShellResult {
    // return sh!("git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@'");
    return sh!("git rev-parse --abbrev-ref origin/HEAD");
}

/// Fully qualified path to the base folder of the repo
pub fn repo_absolute_path() -> String {
    return match sh!("git rev-parse --show-toplevel") {
        Ok(path) => path,
        Err(_) => { // todo - this doesn't seem to actually fail?
            panic!("todo - error about the path being wrong!");
        }
    }
}