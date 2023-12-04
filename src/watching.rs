use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;

use crate::filters::PathFilter;

#[derive(Debug, Clone, Copy)]
pub struct ChangeEvent;

pub trait PathWatcher {
    fn watch(&self, changes: mpsc::SyncSender<ChangeEvent>) -> Result<()>;
}

pub struct NotifyWatcher<F: PathFilter> {
    paths: Vec<PathBuf>,
    filter: F,
}

impl<F: PathFilter> PathWatcher for NotifyWatcher<F> {
    fn watch(&self, changes: mpsc::SyncSender<ChangeEvent>) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for path in &self.paths {
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        }

        let change = ChangeEvent;
        let filtered_paths: HashSet<PathBuf> = HashSet::from_iter(self.filter.paths());

        for res in rx {
            match res {
                Ok(event) => {
                    log::debug!("Change in: {:?}", event.paths);
                    for path in event.paths {
                        if !filtered_paths.contains(&path) {
                            log::debug!("ignored");
                            continue;
                        };

                        if changes.try_send(change).is_err() {
                            log::debug!("buffer full, skipping event");
                        };
                    }
                }
                Err(error) => log::error!("Error: {error:?}"),
            }
        }

        Ok(())
    }
}

impl<F: PathFilter> NotifyWatcher<F> {
    pub fn new(paths: Vec<PathBuf>, filter: F) -> Self {
        NotifyWatcher { paths, filter }
    }
}
