use crate::{SolarError, ToolTrait};
use clap::Parser;
use std::path::PathBuf;
use rust_terminal::Terminal;

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Vhooks {
    /// Git hooks directory name.
    #[arg(short, long, default_value = ".hooks")]
    name: String,

    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,
}

impl ToolTrait for Vhooks {
    fn install(&self) -> Result<(), SolarError> {
        Terminal::command().piped().run("git", vec!["config", "core.hooksPath", ""])?;
        Ok(())
    }

    fn remove(&self) -> Result<(), SolarError> {
        Ok(())
    }
}
