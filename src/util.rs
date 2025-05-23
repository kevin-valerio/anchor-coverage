use anyhow::{ensure, Result};
use std::fs::canonicalize;
use std::process::Command;
use std::{
    env::current_dir,
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn files_with_extension(dir: impl AsRef<Path>, extension: &str) -> Result<Vec<PathBuf>> {
    let mut pcs_paths = Vec::new();
    for result in read_dir(dir)? {
        let entry = result?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new(extension)) {
            pcs_paths.push(path);
        }
    }
    Ok(pcs_paths)
}

pub trait StripCurrentDir {
    fn strip_current_dir(&self) -> &Self;
}

impl StripCurrentDir for Path {
    fn strip_current_dir(&self) -> &Self {
        let Ok(current_dir) = current_dir() else {
            return self;
        };
        self.strip_prefix(current_dir).unwrap_or(self)
    }
}

pub fn which(filename: &str) -> Result<PathBuf> {
    let mut command = Command::new("which");
    let output = command.arg(filename).output()?;
    ensure!(output.status.success(), "command failed: {command:?}");
    let stdout = std::str::from_utf8(&output.stdout)?;
    let path = canonicalize(stdout.trim_end())?;
    Ok(path)
}
