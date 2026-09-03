use clap::Parser;
use solar_utils::working_dir;

use crate::{
    components::InstallableComponent,
    solar_error::SolarError,
    traits::{Installable, Run},
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
