use std::process::ExitCode;

use fastbrew_cli::{HelloArgs, TopLevelArgs};

pub fn run(args: HelloArgs, top_level: &TopLevelArgs) -> ExitCode {
    let greeting = match args.greeting_target() {
        Some(name) => format!("hello, {name}!"),
        None => "hello".to_owned(),
    };

    let output = if top_level.verbose > 0 {
        format!("[v{}] {greeting}", top_level.verbose)
    } else {
        greeting
    };

    println!("{output}");
    ExitCode::SUCCESS
}
