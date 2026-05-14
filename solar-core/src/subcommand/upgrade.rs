use std::path::PathBuf;

use clap::Parser;

use crate::{Action, Tool, SolarError};

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
    pub fn run(&self) -> Result<(), SolarError> {
        Tool::perform(&self.tool, Action::UPGRADE)
    }
}
