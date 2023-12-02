use anyhow::Result;
use clap::Parser;
use filters::PathFilter;
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
    let status = args.status;
    let shell = args.status;
    let clear = args.clear;

    logging::init("info");

    let (tx, rx) = mpsc::sync_channel(0);

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

    let handler = if shell {
        handling::CommandEventHandler::new_shell(command, status)
    } else {
        handling::CommandEventHandler::new(command, status)
    };

    start(handler, rx, clear)
}

fn start<T: handling::EventHandler>(
    handler: T,
    rx: mpsc::Receiver<watching::ChangeEvent>,
    clear: bool,
) -> Result<()> {
    if clear {
        clearscreen::clear()?;
    }
    handler.handle(watching::ChangeEvent)?;

    for ev in rx {
        if clear {
            clearscreen::clear()?;
        }
        handler.handle(ev)?;
    }

    Ok(())
}
