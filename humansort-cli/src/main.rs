use std::{
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use humansort_lib::HumansortState;

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
        /// File containing a line-delimited list of items to be sorted
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
        /// Number of items to prompt the user to sort in a single iteration
        #[arg(value_name = "NUM_ITEMS")]
        num_items: Option<u8>,
    },
    /// Reads a humansort file and outputs a sorted list
    Output {
        /// Humansort file to be printed
        #[arg(value_name = "INFILE")]
        hs_file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start {
            input_file,
            hs_file,
        } => {
            // Read input file.
            let infile: Vec<String> = read_to_string(input_file.clone())?
                .lines()
                .map(|s| s.to_string())
                .collect();

            // Convert input file to humansort state.
            let humansort: HumansortState = infile.into();

            // Write the humansort state to the output file.
            let output = serde_json::to_string_pretty(&humansort)?;
            let output_file = match hs_file {
                Some(o) => o,
                // If the user didn't supply an output path, use the input
                // file's path with .humansort appended to it.
                None => {
                    let mut o = input_file.into_os_string();
                    o.push(".humansort");
                    o.into()
                }
            };
            write(output_file, output)?;
        }
        Commands::Sort { hs_file, num_items } => {
            // Read and parse humansort file.

            // Prompt the user for sorting information.
            todo!()
        }
        Commands::Output { hs_file } => {
            // Read and parse humansort file.

            // Print all items in descending order by rating.
            todo!()
        }
    };

    Ok(())
}
