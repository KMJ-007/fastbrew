use clap::Subcommand;

mod hello;

pub use hello::HelloArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Hello(HelloArgs),
}
