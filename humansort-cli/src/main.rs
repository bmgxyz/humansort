use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reads a line-delimited list of items and creates a humansort file
    Start {
        /// File containing a line-delimeted list of items to be sorted
        #[arg(value_name = "INFILE")]
        input_file: PathBuf,
        /// Name of the humansort file to be created (defaults to
        /// <INFILE>.humansort)
        #[arg(value_name = "OUTFILE")]
        hs_file: Option<PathBuf>,
    },
    /// Read a humansort file and interactively sort it
    Sort {
        /// Humansort file to be sorted
        #[arg(value_name = "INFILE")]
        hs_file: PathBuf,
    },
    /// Reads a humansort file and outputs a sorted list
    Output {
        /// Humansort file to be printed
        #[arg(value_name = "INFILE")]
        hs_file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
}
