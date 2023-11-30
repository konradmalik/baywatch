use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub struct PathWatcher {
    paths: Vec<PathBuf>,
}

impl PathWatcher {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        PathWatcher { paths }
    }

    pub fn watch(&self) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for path in &self.paths {
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        }

        for res in rx {
            match res {
                Ok(event) => {
                    log::debug!("Change in: {:?}", event.paths);
                    println!("Change in: {:?}", event.paths)
                }
                Err(error) => log::error!("Error: {error:?}"),
            }
        }

        Ok(())
    }
}
