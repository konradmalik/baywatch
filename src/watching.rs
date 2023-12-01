use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::mpsc;

pub struct ChangeEvent {
    path: PathBuf,
}

impl ChangeEvent {
    pub fn new() -> Self {
        ChangeEvent {
            path: PathBuf::new(),
        }
    }
}

impl Display for ChangeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.display().fmt(f)
    }
}

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

        for res in rx {
            match res {
                Ok(event) => {
                    log::debug!("Change in: {:?}", event.paths);
                    for path in event.paths {
                        let change = ChangeEvent { path };
                        if changes.try_send(change).is_err() {
                            log::debug!("buffer full, ignoring event");
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
