use std::io;
use std::path::Path;
use std::process::Command;

pub fn run_programm(programm: &Path, args: &[&str]) -> io::Result<()> {
    Command::new(programm).args(args).status()?;

    Ok(())
}
