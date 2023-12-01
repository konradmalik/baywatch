use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub struct ChangeEvent;

pub trait PathWatcher {
    fn watch(&self, changes: mpsc::SyncSender<ChangeEvent>) -> Result<()>;
}

pub struct NotifyWatcher {
    paths: Vec<PathBuf>,
}

impl PathWatcher for NotifyWatcher {
    fn watch(&self, changes: mpsc::SyncSender<ChangeEvent>) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for path in &self.paths {
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        }

        let change = ChangeEvent;

        for res in rx {
            match res {
                Ok(event) => {
                    log::debug!("Change in: {:?}", event.paths);
                    for path in event.paths {
                        if changes.try_send(change).is_err() {
                            log::debug!("buffer full, ignoring event in: {}", path.display());
                        };
                    }
                }
                Err(error) => log::error!("Error: {error:?}"),
            }
        }

        Ok(())
    }
}

impl NotifyWatcher {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        NotifyWatcher { paths }
    }
}
