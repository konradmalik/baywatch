use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Paths to watch
    #[arg(short, long, default_value = PathBuf::from(".").into_os_string())]
    pub path: Vec<PathBuf>,
}
