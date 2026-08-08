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
    /// Zip archive operations.
    Zip {
        #[clap(subcommand)]
        command: ZipCommand,
    },

    /// Base64 operations.
    Base64 {
        #[clap(subcommand)]
        command: Base64Command,
    },

    /// Unleash.
    #[cfg(windows)]
    Unleash {
        /// Target.
        #[clap(long, short)]
        target: PathBuf,

        /// Recursive.
        #[clap(long, short)]
        recursive: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ZipCommand {
    /// Compress zip archive.
    Compress {
        /// Compression target
        #[clap(long, short)]
        input: PathBuf,

        /// Zip archive path.
        #[clap(long, short)]
        output: PathBuf,
    },

    /// Expand zip archive.
    Expand {
        /// Zip archive path.
        #[clap(long, short)]
        input: PathBuf,

        /// Expand destination.
        #[clap(long, short)]
        output: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum Base64Command {
    /// Encode file as base64.
    Encode {
        /// Source.
        #[clap(long, short)]
        input: PathBuf,

        /// Destination.
        #[clap(long, short)]
        output: PathBuf,
    },

    /// Decode file as base64.
    Decode {
        /// Source.
        #[clap(long, short)]
        input: PathBuf,

        /// Destination.
        #[clap(long, short)]
        output: PathBuf,
    },
}

impl Action {
    fn handle(self) -> i32 {
        match self {
            Action::Zip { command } => command.handle(),
            Action::Base64 { command } => command.handle(),
            #[cfg(windows)]
            Action::Unleash { target, recursive } => {
                let result = if recursive {
                    nazrin::unleash::unleash_recursive(&target)
                } else {
                    nazrin::unleash::unleash(&target)
                };
                handle_result(result)
            }
        }
    }
}

impl ZipCommand {
    fn handle(self) -> i32 {
        match self {
            ZipCommand::Compress { input, output } => {
                let result = nazrin::zip::compress(&input, &output);
                handle_result(result)
            }
            ZipCommand::Expand { input, output } => {
                let result = nazrin::zip::expand(&input, &output);
                handle_result(result)
            }
        }
    }
}

impl Base64Command {
    fn handle(self) -> i32 {
        match self {
            Base64Command::Encode { input, output } => {
                let result = nazrin::base64::encode(&input, &output);
                handle_result(result)
            }
            Base64Command::Decode { input, output } => {
                let result = nazrin::base64::decode(&input, &output);
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
