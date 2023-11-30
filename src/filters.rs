use std::path::PathBuf;

pub trait PathFilter {
    fn paths(&self) -> Vec<PathBuf>;
}

pub struct IgnorePathFilter {
    root: PathBuf,
}

impl PathFilter for IgnorePathFilter {
    fn paths(&self) -> Vec<PathBuf> {
        ignore::Walk::new(&self.root)
            .flatten()
            .filter(|d| d.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|d| d.into_path())
            .collect()
    }
}

impl IgnorePathFilter {
    pub fn new(root: PathBuf) -> Self {
        IgnorePathFilter { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_include_this_file() {
        let root_path = get_root_path();
        let filter = IgnorePathFilter::new(root_path.to_owned());
        let paths = filter.paths();
        let this_file = root_path.join("src/filters.rs");
        assert!(paths.contains(&this_file));
    }

    #[test]
    fn test_does_not_include_git() {
        let root_path = get_root_path();
        let filter = IgnorePathFilter::new(root_path.to_owned());
        let paths = filter.paths();
        let git = root_path.join(".git");
        assert!(!paths.contains(&git));
    }

    #[test]
    fn test_any_files_are_returned() {
        let filter = get_in_root();
        let paths = filter.paths();
        assert!(!paths.is_empty())
    }

    #[test]
    fn test_non_files_are_not_returned() {
        let filter = get_in_root();
        let paths = filter.paths();
        let non_files = paths.iter().filter(|p| !p.is_file()).count();
        assert_eq!(non_files, 0)
    }

    fn get_in_root() -> IgnorePathFilter {
        IgnorePathFilter::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    }

    fn get_root_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }
}
