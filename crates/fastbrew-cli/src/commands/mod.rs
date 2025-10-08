use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Hello(HelloArgs),
    #[command(alias = "i")]
    Install(InstallArgs),
}

#[derive(Clone, Debug, Args)]
pub struct HelloArgs {
    #[arg(long)]
    pub name: Option<String>,
}

impl HelloArgs {
    pub fn greeting_target(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Clone, Debug, Args)]
pub struct InstallArgs {
    #[arg(value_name = "PACKAGES")]
    pub package_name: Vec<String>,
}
