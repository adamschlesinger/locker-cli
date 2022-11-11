use std::io;
use crate::sh;
use crate::shell::{ShellResult};

/// todo
pub fn install() -> ShellResult {
    return sh!("git lfs install");
}

/// todo
pub fn track_type(file_ext: &str) -> ShellResult {
    return sh!("git lfs track \"*.{file_ext}\" --lockable");
}

/// todo
pub fn track_file(path: &str) -> ShellResult {
    return sh!("git lfs track --filename \"{path}\" --lockable");
}

/// todo
pub fn lock(path: &str) -> ShellResult {
    return sh!("git lfs lock \"{path}\"");
}

/// todo
pub fn unlock(path: &str, force: bool) -> ShellResult {
    return sh!("git lfs unlock \"{path}\"");
}

/// todo
pub fn locked_files(path: &str) -> ShellResult {
    return sh!("git lfs lock \"{path}\"");
}

/// todo
pub fn check_installed() -> io::Result<bool> {
    // let stdout = LFS::cmd("env")?.stdout;
    // let env_string = String::from_utf8(stdout)?;

    // todo - how to do this?
    let installed = true;

    println!("LFS installation status: {installed}");
    return Ok(installed);
}