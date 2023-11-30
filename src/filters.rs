use ignore::Walk;
use std::path::{Path, PathBuf};

pub fn get_not_ignored_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    Walk::new(&path)
        .flatten()
        .filter(|d| d.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|d| d.into_path())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_include_this_file() {
        let root_path = get_root_path();
        let paths = get_not_ignored_files(&root_path);
        let this_file = root_path.join("src/filters.rs");
        assert!(paths.contains(&this_file));
    }

    #[test]
    fn test_does_not_include_git() {
        let root_path = get_root_path();
        let paths = get_not_ignored_files(&root_path);
        let git = root_path.join(".git");
        assert!(!paths.contains(&git));
    }

    #[test]
    fn test_any_files_are_returned() {
        let root_path = get_root_path();
        let paths = get_not_ignored_files(root_path);
        assert!(!paths.is_empty())
    }

    #[test]
    fn test_non_files_are_not_returned() {
        let root_path = get_root_path();
        let paths = get_not_ignored_files(root_path);
        let non_files = paths.iter().filter(|p| !p.is_file()).count();
        assert_eq!(non_files, 0)
    }

    fn get_root_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }
}
