use super::path::find_in_path;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn run_program(program: &Path, name: &str, args: &[String]) -> io::Result<()> {
    Command::new(program).arg0(name).args(args).status()?;

    Ok(())
}

pub fn run_external(name: &str, args: &[String]) {
    match find_in_path(name) {
        Some(path) => {
            if let Err(err) = run_program(&path, name, args) {
                eprintln!("failed to run program: {err}");
            }
        }
        None => eprintln!("{}: not found", name),
    }
}
