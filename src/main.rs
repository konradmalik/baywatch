use anyhow::Result;
use clap::Parser;
use filters::MultiFilter;
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
    let defer = args.defer;

    logging::init("info");

    let (tx, rx) = mpsc::sync_channel(0);

    thread::spawn(|| {
        let mut filters = Vec::new();
        for path in paths.clone() {
            log::debug!("Watching {}", &path.display());
            let filter = filters::IgnorePathFilter::new(path);
            filters.push(filter);
        }

        let filter = MultiFilter::new(filters);
        let watcher = watching::NotifyWatcher::new(paths, filter);
        watcher.watch(tx).expect("cannot start watcher")
    });

    let handler = if shell {
        handling::CommandEventHandler::new_shell(command, status)
    } else {
        handling::CommandEventHandler::new(command, status)
    };

    start(handler, rx, clear, defer)
}

fn start<T: handling::EventHandler>(
    handler: T,
    rx: mpsc::Receiver<watching::ChangeEvent>,
    clear: bool,
    defer: bool,
) -> Result<()> {
    if clear {
        clearscreen::clear()?;
    }
    if !defer {
        handler.handle(watching::ChangeEvent)?;
    }

    for ev in rx {
        if clear {
            clearscreen::clear()?;
        }
        handler.handle(ev)?;
    }

    Ok(())
}
