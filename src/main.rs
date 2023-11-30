use anyhow::Result;
use clap::Parser;
use ignore::Walk;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    /// The path to watch
    #[arg(default_value = PathBuf::from(".").into_os_string())]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Watching {}", &path.display());

    let paths = get_not_ignored_files(path);

    if let Err(error) = watch(&paths) {
        log::error!("Error: {error:?}");
    }
}

fn get_not_ignored_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    Walk::new(&path)
        .flatten()
        .filter(|d| d.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|d| d.into_path())
        .collect()
}

fn watch<P: AsRef<Path>>(paths: &[P]) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    for path in paths {
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    }

    for res in rx {
        match res {
            Ok(event) => {
                log::info!("Change: {:?}", event.paths)
            }
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}
