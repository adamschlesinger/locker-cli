use std::io;
use std::process::{Command, Output};

pub struct LFS;

fn sh_cmd(cmd:String) -> io::Result<Output> {
    return Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();
}

impl LFS {
    /// todo
    fn cmd(cmd:String) -> io::Result<Output> {
        return sh_cmd(format!("git lfs {cmd}"))
    }

    /// todo
    pub fn install() -> io::Result<Output> {
        return LFS::cmd(String::from("install"));
    }

    pub fn track_type(file_ext:&str) -> io::Result<Output> {
        return LFS::cmd(format!("track \"*.{file_ext}\" --lockable"));
    }

    pub fn track_file(path:&str) -> io::Result<Output> {
        return LFS::cmd(format!("track --filename \"{path}\" --lockable"));
    }

    pub fn lock(path:&str) -> io::Result<Output> {
        return LFS::cmd(format!("lock \"{path}\""));
    }

    pub fn unlock(path:&str, force:bool) -> io::Result<Output> {
        return LFS::cmd(format!("unlock \"{path}\""));
    }

    pub fn locked_files(path:&str) -> io::Result<Output> {
        return LFS::cmd(format!("lock \"{path}\""));
    }

    /// todo
    pub fn check_installed() -> io::Result<bool> {
        // let stdout = LFS::cmd("env")?.stdout;
        // let env_string = String::from_utf8(stdout)?;

        // todo - how to do this?
        let installed = true;

        println!("LFS installation status: {}", installed);
        return Ok(installed);
    }
}
