use crate::path;

const BUILTINS: &[&str] = &["echo", "exit", "type", "pwd", "cd"];

pub fn builtin_cd(args: &[String]) {
    let Some(target_dir) = args.first() else {
        return;
    };
    match path::change_directory(target_dir) {
        Ok(()) => {}
        Err(_) => eprintln!("cd: {}: No such file or directory", target_dir),
    }
}

pub fn builtin_echo(args: &[String]) {
    println!("{}", args.join(" "));
}

pub fn builtin_type(args: &[String]) {
    let Some(target) = args.first() else {
        return;
    };
    if BUILTINS.contains(&target.as_str()) {
        println!("{} is a shell builtin", target);
    } else {
        match path::find_in_path(target) {
            Some(p) => println!("{} is {}", target, p.display()),
            None => eprintln!("{}: not found", target),
        }
    }
}

pub fn builtin_pwd() {
    match path::current_directory() {
        Ok(dir) => println!("{}", dir.display()),
        Err(err) => eprintln!("error: {}", err),
    }
}

