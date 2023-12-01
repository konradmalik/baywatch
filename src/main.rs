use anyhow::Result;
use clap::Parser;
use filters::PathFilter;
use handling::handle_event;
use std::{sync::mpsc, thread};
use watching::PathWatcher;
mod cli;
mod filters;
mod handling;
mod logging;
mod watching;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let path = args.path;

    logging::init("info");

    let (tx, rx) = mpsc::sync_channel(1);

    thread::spawn(|| {
        log::debug!("Watching {}", &path.display());

        let filter = filters::IgnorePathFilter::new(path);
        let watcher = watching::NotifyWatcher::new(filter.paths());

        watcher.watch(tx).expect("cannot start watcher")
    });

    for ev in rx {
        handle_event(ev);
    }

    Ok(())
}
