mod commands;

use clap::Parser;

pub use commands::{Commands, HelloArgs, InstallArgs};

#[derive(Debug, Parser)]
#[command(name = "fastbrew")]
#[command(about = "An experimental fast Homebrew reimplementation.")]
#[command(disable_help_subcommand = true, disable_version_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub top_level: TopLevelArgs,
}

#[derive(Clone, Debug, Default, clap::Args)]
pub struct TopLevelArgs {
    /// Increase output verbosity (add multiple times for more detail).
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,
}
