use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::{SolarError, config::ProjConfig, solar_init};

#[derive(Parser, Clone)]
pub struct New {
    /// The name of the new project.
    name: String,

    /// The project configuration to initialize.
    #[arg(default_value = "cargobinbasic")]
    project: ProjConfig,

    /// The destination to create the new project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl New {
    pub fn run(&mut self) -> Result<(), SolarError> {
        // Ensure the destination directory exists
        let project_dir = self.destination.join(&self.name);
        fs::create_dir_all(&project_dir)?;

        // Initialize the project
        solar_new(&self.project, &mut self.destination, &self.name)
    }
}

pub fn solar_new(
    config: &ProjConfig,
    destination: &mut Path,
    name: &str,
) -> Result<(), SolarError> {
    // Ensure the destination directory exists
    let project_dir = destination.join(name);
    fs::create_dir_all(&project_dir)?;

    // Initialize the project
    solar_init(config, destination)
}
