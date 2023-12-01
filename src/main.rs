use anyhow::Result;
use clap::Parser;
use filters::PathFilter;
use handling::EventHandler;
use std::{sync::mpsc, thread};
use watching::PathWatcher;
mod cli;
mod filters;
mod handling;
mod logging;
mod watching;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let paths = args.path;
    let command = args.command;

    logging::init("info");

    let (tx, rx) = mpsc::sync_channel(1);

    thread::spawn(|| {
        let mut paths_to_watch = Vec::new();
        for path in paths {
            log::debug!("Watching {}", &path.display());
            let filter = filters::IgnorePathFilter::new(path);
            paths_to_watch.extend_from_slice(&filter.paths());
        }

        let watcher = watching::NotifyWatcher::new(paths_to_watch);
        watcher.watch(tx).expect("cannot start watcher")
    });

    let handler = handling::CommandEventHandler::new(command);
    for ev in rx {
        handler.handle(ev);
    }

    Ok(())
}
