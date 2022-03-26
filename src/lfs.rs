use std::io::{Result};
use std::process::{Command, Output};

/// todo
pub fn lfs_cmd(cmd:&str) -> Result<Output> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git lfs {cmd}"))
        .output()
}

/// Check if LFS has already been installed in current repo
pub fn lfs_installer() {

    true;
}