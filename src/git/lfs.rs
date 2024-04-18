use crate::terminal::shell::Result;
use std::io;
use crate::sh;


/// todo
pub fn version() -> Result {
    sh!("git lfs -v")
}

/// todo
pub fn install() -> Result {
    sh!("git lfs install")
}

/// Add a file type to Git attributes to be tracked by Git LFS
/// ```
/// lfs::track_type("png")
/// ```
pub fn track_type(file_ext: &str) -> Result {
    sh!("git lfs track \"*.{file_ext}\" --lockable")
}

/// Add a file to Git attributes to be tracked by Git LFS
/// ```
/// lfs::track_file("foo.png")
/// ```
pub fn track_file(path: &str) -> Result {
    sh!("git lfs track --filename \"{path}\" --lockable")
}

/// todo
/// ```
/// lfs::lock("foo.png")
/// ```
pub fn lock(path: &str) -> Result {
    sh!("git lfs lock \"{path}\"")
}

/// todo
/// ```
/// lfs::unlock("foo.png")
/// ```
pub fn unlock(path: &str, force: bool) -> Result {
    sh!("git lfs unlock \"{path}\"")
}

/// todo
pub fn locked_files() -> Result {
    sh!("git lfs locks")
}

/// todo
pub fn check_installed() -> io::Result<bool> {
    // let stdout = LFS::cmd("env")?.stdout;
    // let env_string = String::from_utf8(stdout)?;

    // todo - how to do this?
    let installed = true;

    println!("LFS installation status: {installed}");
    Ok(installed)
}
