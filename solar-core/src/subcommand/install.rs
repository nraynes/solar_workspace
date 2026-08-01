use clap::Parser;

use crate::{
    components::InstallableComponent,
    solar_error::SolarError,
    traits::{Installable, Run},
    working_dir,
};

#[derive(Parser, Clone)]
pub struct Install {
    /// The name of the component to install.
    #[command(subcommand)]
    component: InstallableComponent,
}

impl Run for Install {
    fn run(&self) -> Result<(), SolarError> {
        self.component.install(&working_dir()?)
    }
}
