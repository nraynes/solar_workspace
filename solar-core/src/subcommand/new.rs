use clap::Parser;

use crate::{
    project::Project,
    solar_error::SolarError,
    traits::{ConfigureProject, Run},
    working_dir,
};

#[derive(Parser, Clone)]
pub struct New {
    /// The project configuration to create.
    #[command(subcommand)]
    project: Project,

    /// The name of the new project to create.
    #[arg(short, long)]
    name: String,
}

impl Run for New {
    fn run(&self) -> Result<(), SolarError> {
        self.project.new(&working_dir()?, &self.name)
    }
}
