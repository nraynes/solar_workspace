use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Install {
    /// The name of the tool to install.
    #[command(subcommand)]
    tool: Tool,
}

impl Install {
    pub fn run(&mut self) -> Result<(), SolarError> {
        self.tool.act(&Action::INSTALL, None)
    }
}
