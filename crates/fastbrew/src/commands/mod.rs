use std::process::ExitCode;

use fastbrew_cli::{Commands, TopLevelArgs};

pub mod tools;

pub fn dispatch(command: Commands, top_level: &TopLevelArgs) -> ExitCode {
    match command {
        Commands::Hello(args) => tools::hello::run(args, top_level),
        Commands::Install(args) => tools::install::run(args, top_level),
    }
}
