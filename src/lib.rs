use log::trace;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileLookupError {
    CwdNotFound,
    HomeDirNotFound,
}

impl Error for FileLookupError {}

impl fmt::Display for FileLookupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileLookupError::CwdNotFound => write!(f, "Error retrieving current working directory"),
            FileLookupError::HomeDirNotFound => write!(f, "Error retrieving home directory"),
        }
    }
}

pub fn find_file(file: &str, under: &PathBuf) -> Result<Option<PathBuf>, FileLookupError> {
    let file: PathBuf = file.into();
    // Check if the file is in the cwd
    if file.exists() {
        trace!("Found {:?} in cwd", &file);
        return Ok(Some(file));
    };
    // Find the current working directory
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err(FileLookupError::CwdNotFound),
    };
    for dir in cwd.ancestors() {
        trace!("Looking in {:?}", dir);
        // If we're above the `under` directory (e.g. $HOME),
        // stop looking
        if !dir.starts_with(under) {
            trace!("File {:?} not found under {:?}", &file, dir);
            return Ok(None);
        }
        // Check for file
        // TODO: Could possibly use set_file_name here to avoid extra allocation
        let file_path = dir.with_file_name(&file);
        trace!("Checking for existence of path {:?}", file_path);
        if file_path.exists() {
            return Ok(Some(file_path));
        }
    }
    // Search terminates
    trace!("File {:?} not found anywhere", file);
    Ok(None)
}

pub fn home_find_file(file: &str) -> Result<Option<PathBuf>, FileLookupError> {
    // Find the home directory
    let home: PathBuf = match dirs::home_dir() {
        Some(path) => {
            trace! {"Found home directory {:?}", &path};
            path
        }
        None => return Err(FileLookupError::HomeDirNotFound),
    };

    find_file(file, &home)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_find_file_cargo_toml() {
        home_find_file("Cargo.toml").unwrap();
    }
}
