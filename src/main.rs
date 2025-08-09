use anyhow::Result;
use clap::Parser;

mod cli;

use cli::{Cli, Commands};
fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Path: {:?}", cli.path);
    println!("Exclude: {:?}", cli.exclude);
    println!("Show hidden: {}", cli.show_hidden);
    println!("Output JSON: {:?}", cli.output_json);
    println!("Depth: {}", cli.depth);

    match &cli.command {
        Some(cli::Commands::ListDisks) => {
            println!("Subcommand: ListDisks");
        }
        Some(cli::Commands::Analyze { path, format }) => {
            println!("Subcommand: Analyze");
            println!("  Path: {:?}", path);
            println!("  Format: {}", format);
        }
        None => {
            println!("No subcommand");
        }
    }

    match &cli.command {
        Some(Commands::ListDisks) => {
            println!("Listing disks is not yet implemented.");
            // TODO: Implement disk listing logic here
            Ok(())
        }
        Some(Commands::Analyze { path, format }) => {
            println!(
                "Analyzing {} with format {} is not yet implemented.",
                path.display(),
                format
            );
            // TODO: Implement analyze logic here (without TUI)
            Ok(())
        }
        None => {
            println!("待实现");
            Ok(())
        }
    }
}
