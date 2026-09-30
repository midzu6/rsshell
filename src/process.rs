use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn run_programm(programm: &Path, name: &str, args: &[&str]) -> io::Result<()> {
    Command::new(programm)
    .arg0(name)
    .args(args)
    .status()?;

    Ok(())
}
