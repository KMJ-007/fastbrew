use clap::Args;

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
