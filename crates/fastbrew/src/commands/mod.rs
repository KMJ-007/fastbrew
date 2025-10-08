use std::process::ExitCode;

use fastbrew_cli::{Commands, TopLevelArgs};

pub mod hello;

pub fn dispatch(command: Commands, top_level: &TopLevelArgs) -> ExitCode {
    match command {
        Commands::Hello(args) => hello::run(args, top_level),
    }
}
