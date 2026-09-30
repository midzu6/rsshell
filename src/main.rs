use parser::{Command, parse_command};
use std::{
    io::{self, Write},
    ops::ControlFlow,
};

mod builtins;
mod parser;
mod path;
mod process;

fn execute(cmd: Command) -> ControlFlow<()> {
    match cmd {
        Command::Exit => return ControlFlow::Break(()),
        Command::Cd(args) => builtins::builtin_cd(&args),
        Command::Echo(args) => builtins::builtin_echo(&args),
        Command::Pwd => builtins::builtin_pwd(),
        Command::Type(args) => builtins::builtin_type(&args),
        Command::External { name, args } => process::run_external(&name, &args),
    }
    ControlFlow::Continue(())
}

fn read_input() -> Option<String> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(n) => {
            if n == 0 {
                None
            } else {
                Some(buf)
            }
        }
        Err(err) => panic!("err: {}", err),
    }
}

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let Some(line) = read_input() else {
            break;
        };

        let Some(cmd) = parse_command(&line) else {
            continue;
        };

        if execute(cmd).is_break() {
            break;
        }
    }
}

fn main() {
    run();
}
