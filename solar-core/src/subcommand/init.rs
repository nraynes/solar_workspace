use std::{
    fs::{self, File},
    path::PathBuf,
};

use rust_terminal::Terminal;

use clap::Parser;

use crate::{Install, SolarError, Global};

#[derive(Parser, Clone)]
pub struct Init {
    /// The destination to initialize the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Init {
    pub fn run(&self) -> Result<(), SolarError> {
        Global::git_init(&self.destination)?;

        // Create a README.md file
        File::create(&self.destination.join(PathBuf::from("README.md")))?;

        // Install all tools into the project
        let installer: Install = Install::parse_from(vec![
            "",
            self.destination
                .to_str()
                .ok_or("Failed to convert destination path to string.")?,
        ]);
        installer.run()?;
        Ok(())
    }
}
