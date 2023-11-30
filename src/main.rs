use anyhow::Result;
use clap::Parser;
mod cli;
mod filters;
mod logging;
mod watching;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let path = args.path;

    logging::init("info");

    log::debug!("Watching {}", &path.display());

    let paths = filters::get_not_ignored_files(path);

    let watcher = watching::PathWatcher::new(paths);
    watcher.watch()
}
