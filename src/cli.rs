use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// The path to watch
    #[arg(default_value = PathBuf::from(".").into_os_string())]
    pub path: std::path::PathBuf,
}
