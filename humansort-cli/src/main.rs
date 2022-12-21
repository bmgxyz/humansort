use std::{
    error::Error,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use console::Term;
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
    New {
        /// File containing a line-delimited list of items to be sorted
        #[arg(value_name = "INFILE")]
        input_file: PathBuf,
        /// Name of the humansort file to be created (defaults to
        /// <INFILE>.humansort)
        #[arg(value_name = "OUTFILE")]
        hs_file: Option<PathBuf>,
    },
    /// Update a humansort file with new inputs; adds new items and removes
    /// missing ones
    Merge {
        /// File containing a line-delimited list of items to be sorted
        #[arg(value_name = "INFILE")]
        input_file: PathBuf,
        /// Name of the humansort file to be updated
        #[arg(value_name = "OUTFILE")]
        hs_file: PathBuf,
    },
    /// Read a humansort file and interactively sort it
    Sort {
        /// Humansort file to be sorted
        #[arg(value_name = "INFILE")]
        hs_file: PathBuf,
        /// Number of items to prompt the user to sort in a single iteration
        #[arg(value_name = "NUM_ITEMS")]
        maybe_num_items: Option<usize>,
    },
    /// Reads a humansort file and outputs a sorted list
    Output {
        /// Humansort file to be printed
        #[arg(value_name = "INFILE")]
        hs_file: PathBuf,
    },
}

fn read_input_file(input_file: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_to_string(input_file)?
        .lines()
        .map(|s| s.to_string())
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            input_file,
            hs_file,
        } => {
            let infile = read_input_file(&input_file)?;

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
        Commands::Merge {
            input_file,
            hs_file,
        } => {
            // Read input file.
            let new_items = read_input_file(&input_file)?;

            // Read and parse humansort file.
            let outfile = read_to_string(hs_file.clone())?;
            let mut humansort = serde_json::from_str::<HumansortState>(&outfile)?;

            // Update the humansort state by deleting missing items and adding
            // new ones.
            humansort.merge(&new_items);

            // Write updated state to the original file.
            let output = serde_json::to_string_pretty(&humansort)?;
            write(hs_file, output)?;
        }
        Commands::Sort {
            hs_file,
            maybe_num_items,
        } => {
            // Read and parse humansort file.
            let infile = read_to_string(hs_file.clone())?;
            let mut humansort = serde_json::from_str::<HumansortState>(&infile)?;

            let num_items = if let Some(n) = maybe_num_items {
                if (2..=9).contains(&n) {
                    humansort.set_num_items(n)?;
                    n
                } else {
                    return Err(format!(
                        "Number of items must be between 2 and 9, inclusive (found {})",
                        n
                    )
                    .into());
                }
            } else {
                humansort.num_items()
            };

            let term = Term::stdout();
            for _ in 0..num_items {
                term.write_line("")?;
            }

            loop {
                // Clear lines.
                term.clear_last_lines(num_items)?;

                // Check for stopping criterion and notify the user as
                // appropriate.
                // TODO

                // Get options and print them.
                let items = humansort.next();
                for (idx, item) in items.iter().enumerate() {
                    term.write_line(&format!("({}) {}", idx + 1, *item))?;
                }

                // Get user's choice.
                let mut choice = ' ';
                while !choice.is_ascii_digit() && choice != 'q' {
                    choice = term.read_char()?;
                }
                // Quit if the user said so.
                if choice == 'q' {
                    term.clear_last_lines(num_items)?;
                    break;
                }
                // Otherwise, try to convert their choice into a number.
                let choice_idx = (choice.to_digit(10).unwrap() - 1) as usize;

                // Update sort state.
                let mut new_data = vec![items[choice_idx].clone()];
                for (idx, item) in items.iter().enumerate() {
                    if idx == choice_idx {
                        continue;
                    }
                    new_data.push(item.clone());
                }
                humansort.update(&new_data)?;

                // Write the new state to the input file.
                let output = serde_json::to_string_pretty(&humansort)?;
                write(hs_file.clone(), output)?;
            }
        }
        Commands::Output { hs_file } => {
            // Read and parse humansort file.
            let infile = read_to_string(hs_file)?;
            let humansort = serde_json::from_str::<HumansortState>(&infile)?;

            // Print all items in descending order by rating.
            let term = Term::stdout();
            for item in humansort {
                // If the output of humansort is a pipe instead of stdout, then
                // the pipe can break before we finish writing our items. (For
                // example, this happens when piping to head.) Rather than
                // throwing an error in that case, we just stop sending items
                // early.
                if term.write_line(&format!("{}", item)).is_err() {
                    break;
                }
            }
        }
    };

    Ok(())
}
