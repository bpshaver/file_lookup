use std::env;
use std::path::PathBuf;

use std::error::Error;
use std::fmt;

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
    // Find the current working directory
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err(FileLookupError::CwdNotFound),
    };
    // Iterate up through parent directories
    for dir in cwd.ancestors() {
        // If we're above the `under` directory (e.g. $HOME),
        // stop looking
        if !dir.starts_with(under) {
            return Ok(None);
        }
        // Check for file
        let file_path = dir.with_file_name(file);
        if file_path.exists() {
            return Ok(Some(file_path));
        }
    }
    // Search terminates
    Ok(None)
}

pub fn home_find_file(file: &str) -> Result<Option<PathBuf>, FileLookupError> {
    // Find the home directory
    let home: PathBuf = match dirs::home_dir() {
        Some(path) => path,
        None => return Err(FileLookupError::HomeDirNotFound),
    };

    find_file(file, &home)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn home_find_file_basic() {
        home_find_file("Cargo.toml").unwrap();
    }
}
