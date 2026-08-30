use clap::Parser;
use solar_utils::working_dir;

use crate::{
    project::Project,
    solar_error::SolarError,
    traits::{ConfigureProject, Run},
};

#[derive(Parser, Clone)]
pub struct Deinit {
    /// The project configuration to deinitialize.
    #[command(subcommand)]
    project: Project,
}

impl Run for Deinit {
    fn run(&self) -> Result<(), SolarError> {
        self.project.deinit(&working_dir()?)
    }
}
