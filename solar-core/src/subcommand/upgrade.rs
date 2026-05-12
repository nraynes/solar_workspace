use std::path::PathBuf;

use clap::Parser;
use semver_common::Alert;

use crate::{Action, Tool};

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
    pub fn run(&self) -> Result<(), Alert> {
        Tool::perform(&self.tool, Action::UPGRADE)
    }
}
