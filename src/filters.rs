use ignore::Walk;
use std::path::{Path, PathBuf};

pub fn get_not_ignored_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    Walk::new(&path)
        .flatten()
        .filter(|d| d.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|d| d.into_path())
        .collect()
}
