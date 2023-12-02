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
    /// Whether to execute the command in shell (read from SHELL env var, fallback to 'sh')
    #[arg(short, long)]
    pub shell: bool,
    /// Whether to show exit status
    #[arg(long)]
    pub status: bool,
    /// Whether to clear the screen before each run
    #[arg(short, long)]
    pub clear: bool,
}
