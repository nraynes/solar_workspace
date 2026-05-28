use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Upgrade {
    /// The name of the tool to upgrade. If none is provided, defaults to all tools.
    #[command(subcommand)]
    tool: Tool,
}

impl Upgrade {
    pub fn run(&mut self) -> Result<(), SolarError> {
        self.tool.act(&Action::UPGRADE, None)
    }
}
