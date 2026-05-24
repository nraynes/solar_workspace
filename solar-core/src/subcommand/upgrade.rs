use std::path::PathBuf;

use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Upgrade {
    /// The name of the tool to upgrade. If none is provided, defaults to all tools.
    #[command(subcommand)]
    tool: Option<Tool>,

    /// The destination to upgrade the tools from.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Upgrade {
    pub fn run(&mut self) -> Result<(), SolarError> {
        Tool::perform(
            self.tool.as_mut(),
            Action::UPGRADE,
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
