use std::io::{self, Write};

mod path;

struct ParseCommand<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

enum Command<'a> {
    Exit,
    Echo(ParseCommand<'a>),
    Type(ParseCommand<'a>),
    External(ParseCommand<'a>),
}

fn parse_command(command: &str) -> Command<'_> {
    let mut parts = command.split_whitespace();

    let name = parts.next().unwrap_or("");
    let mut args = Vec::new();

    while let Some(arg) = parts.next() {
        args.push(arg);
    }

    let command = ParseCommand { name, args };

    match name {
        "exit" => Command::Exit,
        "echo" => Command::Echo(command),
        "type" => Command::Type(command),
        _ => Command::External(command),
    }
}

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        match read_input() {
            Some(command) => {
                let command = command.trim();
                if command.is_empty() {
                    continue;
                }
                match parse_command(command) {
                    Command::Exit => break,
                    Command::Echo(cmd) => println!("{}", cmd.args.join(" ")),
                    Command::Type(cmd) => {
                        let target = match cmd.args.first() {
                            Some(t) => *t,
                            None => {
                                continue;
                            }
                        };

                        match target {
                            "echo" | "exit" | "type" => println!("{} is a shell builtin", target),
                            _ => match path::find_in_path(target) {
                                Some(p) => println!("{} is {}", target, p.display()),
                                None => println!("{}: not found", target),
                            },
                        }
                    }
                    Command::External(cmd) => println!("{}: command not found", cmd.name),
                }
            }
            None => break,
        }
    }
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

fn main() {
    run();
}
