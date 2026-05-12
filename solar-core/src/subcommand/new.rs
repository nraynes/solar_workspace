use std::{fs, path::PathBuf};

use clap::Parser;
use semver_common::Alert;

use crate::Init;

#[derive(Parser, Clone)]
pub struct New {
    /// The name of the new projecct.
    name: String,

    /// The destination to create the new project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl New {
    pub fn run(&self) -> Result<(), Alert> {
        // Ensure the destination directory exists
        let project_dir = self.destination.join(&self.name);
        fs::create_dir_all(&project_dir)?;

        // Initialize the project
        let initializer: Init = Init::parse_from(vec![
            "",
            project_dir
                .to_str()
                .ok_or("Failed to convert destination path to string.")?,
        ]);
        initializer.run()?;

        Ok(())
    }
}
