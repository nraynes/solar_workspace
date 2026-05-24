use std::{fs::File, path::PathBuf};

use clap::Parser;

use crate::{Action, Global, SolarError, Tool};

pub fn initialize_solar(destination: &PathBuf) -> Result<(), SolarError> {
    Global::git_init(destination)?;

    // Create a README.md file
    File::create(destination.join(PathBuf::from("README.md")))?;
    // Install all tools into the project
    Ok(Tool::perform(
        None,
        Action::INSTALL,
        Some(destination.clone()),
        vec![&format!(
            "--destination={}",
            destination
                .to_str()
                .ok_or("Failed to extract argument to tool")?
        )],
    )?)
}

#[derive(Parser, Clone)]
pub struct Init {
    /// The destination to initialize the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Init {
    pub fn run(&mut self) -> Result<(), SolarError> {
        Ok(initialize_solar(&self.destination)?)
    }
}
