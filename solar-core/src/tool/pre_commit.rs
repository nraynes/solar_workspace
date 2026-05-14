use crate::{SolarError, ToolTrait};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct PreCommit {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,
}

impl ToolTrait for PreCommit {
    fn install(&self) -> Result<(), SolarError> {
        Ok(())
    }

    fn remove(&self) -> Result<(), SolarError> {
        Ok(())
    }
}
