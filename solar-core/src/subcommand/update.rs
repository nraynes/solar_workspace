use std::path::{Path, PathBuf};

use clap::Parser;

use crate::{Action, Config, SolarError, ToolTrait};

#[derive(Parser, Clone)]
pub struct Update {
    /// The destination to update the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Update {
    pub fn run(&mut self) -> Result<(), SolarError> {
        solar_update(
            &mut Config::load_from(&self.destination)?,
            &self.destination,
        )
    }
}

pub fn solar_update(config: &mut Config, destination: &Path) -> Result<(), SolarError> {
    config.act(&Action::UPGRADE, Some(destination))
}
