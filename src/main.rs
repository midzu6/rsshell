use std::io::{self, Write};

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        match read_input() {
            Some(command) => {
                if command.trim() == "exit" {
                    break
                }
                println!("{}: command not found", command.trim());
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
