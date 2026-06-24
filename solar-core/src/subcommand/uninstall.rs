use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Uninstall {
    /// The name of the tool to remove. If none is provided, defaults to all tools.
    #[command(subcommand)]
    tool: Tool,
}

impl Uninstall {
    pub fn run(&mut self) -> Result<(), SolarError> {
        self.tool.act(&Action::UNINSTALL, None)
    }
}
