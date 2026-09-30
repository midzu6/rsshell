use ::std::env;
use std::{io, path::PathBuf};

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;

    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(name);

        if candidate.is_file() && is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

pub fn current_directory() -> io::Result<PathBuf> {
    env::current_dir()
}

pub fn change_directory(path: &str) -> io::Result<()> {
    if path.eq("~") {
        let home_path = env::var_os("HOME")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))?;
        return env::set_current_dir(PathBuf::from(home_path));
    }

    if let Some(rest) = path.strip_prefix("~/") {
        let home_path = env::var_os("HOME")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))?;

        let target_dir = PathBuf::from(home_path).join(rest);

        return env::set_current_dir(target_dir);
    }

    env::set_current_dir(path)
}

#[cfg(unix)]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable(path: &std::path::Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "exe" | "bat" | "cmd" | "com"
        ),
        None => false,
    }
}
