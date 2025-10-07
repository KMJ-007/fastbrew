use std::process::ExitCode;

use fastbrew::main as fastbrew_main;

#[allow(unsafe_code)]
fn main() -> ExitCode {
    fastbrew_main(std::env::args_os())
}
