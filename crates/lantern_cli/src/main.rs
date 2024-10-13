use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use color_eyre::eyre::Result;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find unused exports in a project
    UnusedExports {
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },

    /// Find files with re-exports
    FilesWithReExports {
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },

    /// Build a dependency graph for a project
    Depgraph {
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },

    /// Find all file level cycles
    Cycles {
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },

    /// Find affected files in a project
    Affected {
        #[arg(short, long, required = true)]
        entries: Vec<PathBuf>,

        #[arg(short, long, required = true)]
        changed: Vec<PathBuf>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = CLI::parse();

    match &cli.command {
        Commands::UnusedExports { path } => {
            commands::unused_exports::command::run(path).unwrap();
        }
        Commands::FilesWithReExports { path } => {
            commands::files_with_reexports::run(path).unwrap();
        }
        Commands::Depgraph { path } => {
            commands::depgraph::build(path).unwrap();
        }
        Commands::Cycles { path } => {
            commands::cycles::run(path).unwrap();
        }
        Commands::Affected { entries, changed } => {
            println!("Entries: {:?}, Changed: {:?}", entries, changed);
            commands::affected::run(entries, changed).unwrap();
        }
    };

    return Ok(());
}
