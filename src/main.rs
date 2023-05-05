use clap::{Parser, Subcommand};
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

    /// Encode file as base64.
    Base64Encode {
        /// Source.
        #[clap(long, short)]
        input: PathBuf,

        /// Destination.
        #[clap(long, short)]
        output: PathBuf,
    },

    /// Decode file as base64.
    Base64Decode {
        /// Source.
        #[clap(long, short)]
        input: PathBuf,

        /// Destination.
        #[clap(long, short)]
        output: PathBuf,
    },

    // Unleash.
    #[cfg(windows)]
    Unleash {
        /// Target.
        #[clap(long, short)]
        target: PathBuf,
    },
}

impl Action {
    fn handle(self) -> i32 {
        use Action::{Base64Decode, Base64Encode, Unleash, Unzip, Zip};

        match self {
            Unzip { input, output } => {
                let result = nazrin::unzip::expand(&input, &output);
                handle_result(result)
            }
            Zip { input, output } => {
                let result = nazrin::zip::compress(&input, &output);
                handle_result(result)
            }
            Base64Encode { input, output } => {
                let result = nazrin::base64::encode(&input, &output);
                handle_result(result)
            }
            Base64Decode { input, output } => {
                let result = nazrin::base64::decode(&input, &output);
                handle_result(result)
            }
            Unleash { target } => {
                let result = nazrin::unleash::unleash(&target);
                handle_result(result)
            }
        }
    }
}

fn handle_result(result: anyhow::Result<()>) -> i32 {
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
