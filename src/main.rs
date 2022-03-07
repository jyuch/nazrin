use clap::{Parser, Subcommand};
use nazrin::unzip;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[clap(bin_name = "nazrin")]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Expand zip archive.
    Unzip {
        /// Zip archive path.
        #[clap(long, short)]
        input: PathBuf,

        /// Expand destination.
        #[clap(long, short)]
        output: PathBuf,
    },

    /// Compress zip archive.
    Zip {
        /// Compression target
        #[clap(long, short)]
        input: PathBuf,

        /// Zip archive path.
        #[clap(long, short)]
        output: PathBuf,
    },
}

impl Action {
    fn handle(self) -> i32 {
        use Action::{Unzip, Zip};

        match self {
            Unzip { input, output } => {
                let result = unzip::expand(&input, &output);
                handle_result(result)
            }
            Zip { input, output } => {
                let result = nazrin::zip::compress(&input, &output);
                handle_result(result)
            }
        }
    }
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) -> i32 {
    match result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

fn main() {
    let result = Cli::parse().action.handle();
    process::exit(result);
}
