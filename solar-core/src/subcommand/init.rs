use clap::Parser;

use crate::{
    project::Project,
    solar_error::SolarError,
    traits::{ConfigureProject, Run},
    working_dir,
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
