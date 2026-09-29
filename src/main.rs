use std::io::{self, Write};

enum Command<'a> {
    Exit,
    Echo(&'a str),
    Type(&'a str),
    Undefined,
}

fn parse_command(command: &str) -> Command<'_> {

    let mut parts = command.splitn(2, ' ');

    let name = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match name {
        "exit" => Command::Exit,
        "echo" => Command::Echo(args),
        "type" => Command::Type(args),
        &_ => Command::Undefined,
    }
}

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        match read_input() {
            Some(command) => {
                let command = command.trim();
                match parse_command(command) {
                    Command::Exit => break,
                    Command::Echo(args) => println!("{}", args),
                    Command::Type(name) => {
                        match name {
                            "echo" => println!("echo is a shell builtin"),
                            "exit" => println!("exit is a shell builtin"),
                            &_ => println!("{}: not found", name),
                        }
                    },
                    Command::Undefined => println!("{}: command not found", command),
                }
            },
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
