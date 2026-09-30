#[derive(Clone, Copy)]
pub enum State {
    Normal,
    SingleQuoted,
}
pub enum Command {
    Exit,
    Echo(Vec<String>),
    Type(Vec<String>),
    External { name: String, args: Vec<String> },
    Cd(Vec<String>),
    Pwd,
}

pub fn parse_command(line: &str) -> Option<Command> {
    let mut tokens = tokenize(line).into_iter();
    let name = tokens.next()?;
    let args: Vec<String> = tokens.collect();

    Some(match name.as_str() {
        "exit" => Command::Exit,
        "pwd" => Command::Pwd,
        "echo" => Command::Echo(args),
        "type" => Command::Type(args),
        "cd" => Command::Cd(args),
        _ => Command::External { name, args },
    })
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut state = State::Normal;
    let mut current = String::new();
    let mut in_token = false;

    for ch in input.chars() {
        match (state, ch) {
            (State::Normal, '\'') => {
                state = State::SingleQuoted;
                in_token = true;
            }
            (State::Normal, c) if c.is_whitespace() => {
                if in_token {
                    tokens.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            (State::Normal, c) => {
                current.push(c);
                in_token = true
            }
            (State::SingleQuoted, '\'') => {
                state = State::Normal;
            }
            (State::SingleQuoted, c) => {
                current.push(c);
            }
        }
    }

    if in_token {
        tokens.push(current);
    }

    tokens
}
