use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::{Action, Global, SolarError, ToolTrait, config::ProjConfig};

#[derive(Parser, Clone)]
pub struct Init {
    /// The project configuration to initialize.
    #[arg(default_value = "cargobinbasic")]
    project: ProjConfig,

    /// The destination to initialize the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Init {
    pub fn run(&mut self) -> Result<(), SolarError> {
        solar_init(&self.project, &self.destination)
    }
}

pub fn solar_init(config: &ProjConfig, destination: &Path) -> Result<(), SolarError> {
    // Initialize git repository if it's not already.
    Global::git_init(destination)?;

    // Create a README.md file
    File::create(destination.join("README.md"))?;

    // Install project configuration
    config.get().act(&Action::INSTALL, Some(destination))
}
