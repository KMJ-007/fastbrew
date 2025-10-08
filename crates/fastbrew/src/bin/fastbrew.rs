use std::process::ExitCode;

fn main() -> ExitCode {
    fastbrew::main(std::env::args_os())
}
