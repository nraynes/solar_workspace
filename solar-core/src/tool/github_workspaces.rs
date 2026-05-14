use crate::{SolarError, ToolTrait};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Workspaces {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,
}

impl ToolTrait for Workspaces {
    fn install(&self) -> Result<(), SolarError> {
        Ok(())
    }

    fn remove(&self) -> Result<(), SolarError> {
        Ok(())
    }
}
