use std::process::ExitCode;

use fastbrew_cli::{InstallArgs, TopLevelArgs};

pub fn run(args: InstallArgs, _top_level: &TopLevelArgs) -> ExitCode {
    println!("Installing {}", args.package_name.join(", "));
    ExitCode::SUCCESS
}
