use std::path::PathBuf;

use clap::Parser;

use crate::{Action, Tool, SolarError};

#[derive(Parser, Clone)]
pub struct Remove {
    /// The name of the tool to remove. If none is provided, defaults to all tools.
    #[command(subcommand)]
    tool: Option<Tool>,

    /// The destination to remove the tools from.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Remove {
    pub fn run(&self) -> Result<(), SolarError> {
        Tool::perform(&self.tool, Action::REMOVE)
    }
}
