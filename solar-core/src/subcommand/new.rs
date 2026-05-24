use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{SolarError, initialize_solar};

#[derive(Parser, Clone)]
pub struct New {
    /// The name of the new projecct.
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
        Ok(initialize_solar(&mut self.destination)?)
    }
}
