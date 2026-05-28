use std::path::PathBuf;

use clap::Parser;

use crate::{Action, SolarError, Tool};

#[derive(Parser, Clone)]
pub struct Update {
    /// The destination to initialize the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,
}

impl Update {
    pub fn run(&mut self) -> Result<(), SolarError> {
        Ok(solar_update(&self.destination)?)
    }
}

pub fn solar_update(destination: &PathBuf) -> Result<(), SolarError> {
    // Install all tools into the project
    Ok(Tool::act_all(
        Action::UPGRADE,
        Some(destination.clone()),
        vec![&format!(
            "--destination={}",
            destination
                .to_str()
                .ok_or("Failed to extract argument to tool")?
        )],
    )?)
}
