use std::path::PathBuf;

use clap::Parser;
use semver_common::Alert;

use crate::{Action, Tool};

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
    pub fn run(&self) -> Result<(), Alert> {
        Tool::perform(&self.tool, Action::INSTALL)
    }
}
