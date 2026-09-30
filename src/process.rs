use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn run_program(program: &Path, name: &str, args: &[&str]) -> io::Result<()> {
    Command::new(program).arg0(name).args(args).status()?;

    Ok(())
}
