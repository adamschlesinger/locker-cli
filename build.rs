use clap::arg;
use clap_mangen::Man;
use std::env::var_os;
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // let out_dir = PathBuf::from(var_os("OUT_DIR")
    //     .ok_or(ErrorKind::NotFound)?);
    //
    // let cmd = clap::Command::new("mybin")
    //     .arg(arg!(-n --name <NAME>))
    //     .arg(arg!(-c --count <NUM>));
    //
    // let man = Man::new(cmd);
    // let mut buffer: Vec<u8> = Default::default();
    // man.render(&mut buffer)?;
    //
    // std::fs::write(out_dir.join("mybin.1"), buffer)?;
    Ok(())
}
