use std::process::ExitCode;

use fastbrew_cli::{InstallArgs, TopLevelArgs};

pub fn run(args: InstallArgs, _top_level: &TopLevelArgs) -> ExitCode {
    if args.force {
        println!("force downloading");
    }
    println!("Installing {}", args.package_name.join(", "));
    ExitCode::SUCCESS
}
