use crate::sh;
use std::process;

/// Name of the default branch
/// https://stackoverflow.com/questions/28666357/git-how-to-get-default-branch
pub fn origin_default() -> String {
    // return sh!("git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@'");
    match sh!("git rev-parse --abbrev-ref origin/HEAD") {
        Ok(branch) => branch,
        Err(err) => process::exit(err.code),
    }
}

/// Fully qualified path to the base folder of the repo
pub fn repo_absolute_path(input_path: Option<String>) -> String {
    let cmd_output = match input_path {
        Some(input_path) => sh!("git -C {input_path} rev-parse --show-toplevel"),
        None => sh!("git rev-parse --show-toplevel"),
    };

    match cmd_output {
        Ok(path) => path,
        Err(err) => process::exit(err.code),
    }
}
