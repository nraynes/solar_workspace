use clap::Parser;
use solar_utils::working_dir;

use crate::{
    project::Project,
    solar_error::SolarError,
    traits::{ConfigureProject, Run},
};

#[derive(Parser, Clone)]
pub struct Update {
    /// The project configuration to update.
    #[command(subcommand)]
    project: Project,
}

impl Run for Update {
    fn run(&self) -> Result<(), SolarError> {
        self.project.update(&working_dir()?)
    }
}
