use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{SolarError, solar_init};

#[derive(Parser, Clone)]
pub struct New {
    /// The name of the new project.
    name: String,

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
        Ok(solar_new(&mut self.destination, &self.name)?)
    }
}

pub fn solar_new(destination: &mut PathBuf, name: &str) -> Result<(), SolarError> {
    // Ensure the destination directory exists
    let project_dir = destination.join(&name);
    fs::create_dir_all(&project_dir)?;

    // Initialize the project
    Ok(solar_init(destination)?)
}
