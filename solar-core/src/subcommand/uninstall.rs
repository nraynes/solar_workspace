use std::path::PathBuf;

use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Uninstall {
    /// The name of the tool to remove. If none is provided, defaults to all tools.
    #[command(subcommand)]
    tool: Option<Tool>,

    /// The destination to remove the tools from.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Uninstall {
    pub fn run(&mut self) -> Result<(), SolarError> {
        Tool::perform(
            self.tool.as_mut(),
            Action::UNINSTALL,
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
