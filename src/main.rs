use anyhow::Result;
use clap::Parser;
use filters::PathFilter;
use watching::PathWatcher;
mod cli;
mod filters;
mod logging;
mod watching;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let path = args.path;

    logging::init("info");

    log::debug!("Watching {}", &path.display());

    let filter = filters::IgnorePathFilter::new(path);
    let watcher = watching::NotifyWatcher::new(filter.paths());
    watcher.watch()
}
