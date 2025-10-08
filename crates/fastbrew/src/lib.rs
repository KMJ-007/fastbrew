mod commands;

use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use commands::dispatch;
use fastbrew_cli::Cli;
use fastbrew_static::EnvVars;

// Accepts custom argument iterators so integration tests can drive the CLI directly.
pub fn main<I, T>(args: I) -> ExitCode
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    if let Ok(current_exe) = std::env::current_exe() {
        unsafe {
            // SAFETY: The proof obligation must be satisfied by the caller.
            // This will become unsafe in Rust 2024
            // See https://doc.rust-lang.org/std/env/fn.set_var.html#safety
            std::env::set_var(EnvVars::FASTBREW, current_exe);
        }
    }

    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    let Cli { command, top_level } = cli;
    dispatch(command, &top_level)
}
