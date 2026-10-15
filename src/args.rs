use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Regular expression pattern to search for
    #[arg(short, long)]
    pub pattern: String,

    /// Path to the log file
    #[arg(short, long)]
    pub path: PathBuf,
}