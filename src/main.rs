use clap::Parser;
use ignore::Walk;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    /// The path to watch
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let path = args.path.unwrap_or(Path::new(".").to_path_buf());

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("not ignored contents of {}:", path.display());
    for result in Walk::new(&path) {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    log::info!("Watching {}", &path.display());

    if let Err(error) = watch(path) {
        log::error!("Error: {error:?}");
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => log::info!("Change: {event:?}"),
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}
