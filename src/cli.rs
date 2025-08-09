use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "TianYi Zhou",
    version,
    about = "A high-performance, interactive command-line disk space visualization tool.",
    long_about = "DiskTree scans your file system and visualizes disk space usage in a beautiful, dynamic TUI."
)]
pub struct Cli {
    /// Path to scan (defaults to current directory)
    #[clap(default_value = ".")]
    pub path: PathBuf,

    /// Exclude paths matching a regex pattern
    #[clap(short, long, value_name = "PATTERN")]
    pub exclude: Option<String>,

    /// Show hidden files and directories
    #[clap(short, long)]
    pub show_hidden: bool,

    /// Output scan results to a JSON file instead of TUI
    #[clap(short, long, value_name = "FILE")]
    pub output_json: Option<PathBuf>,

    /// Specify the maximum depth for scanning (0 for unlimited)
    #[clap(short, long, default_value_t = 0)]
    pub depth: u32,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available disks/partitions
    ListDisks,
    /// Analyze a specific file or directory without TUI
    Analyze {
        /// Path to analyze
        path: PathBuf,
        /// Output format (json, csv, text)
        #[clap(short, long, default_value = "text")]
        format: String,
    },
}
