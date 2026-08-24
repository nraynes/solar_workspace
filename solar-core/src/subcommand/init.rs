use clap::Parser;
use solar_utils::working_dir;

use crate::{
    project::Project,
    solar_error::SolarError,
    traits::{ConfigureProject, Run},
};

#[derive(Parser, Clone)]
pub struct Init {
    /// The project configuration to initialize.
    #[command(subcommand)]
    project: Project,
}

impl Run for Init {
    fn run(&self) -> Result<(), SolarError> {
        self.project.init(&working_dir()?)
    }
}
