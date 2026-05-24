use std::path::PathBuf;

use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Install {
    /// The name of the tool to install.
    #[command(subcommand)]
    tool: Option<Tool>,

    /// The destination to install the tools to.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Install {
    pub fn run(&mut self) -> Result<(), SolarError> {
        Tool::perform(
            self.tool.as_mut(),
            Action::INSTALL,
            Some(self.destination.clone()),
            vec![&format!(
                "--destination={}",
                self.destination
                    .to_str()
                    .ok_or("Failed to extract argument to tool")?
            )],
        )
    }
}
