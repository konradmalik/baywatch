use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Paths to watch
    #[arg(short, long, default_value = PathBuf::from(".").into_os_string())]
    pub path: Vec<PathBuf>,
    /// Command to execute
    #[arg(required=true, num_args=1..)]
    pub command: Vec<String>,
    /// Whether to show exit status
    #[arg(short, long)]
    pub status: bool,
}
