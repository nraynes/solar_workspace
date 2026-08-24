use clap::Parser;
use solar_utils::working_dir;

use crate::{
    components::UninstallableComponent,
    solar_error::SolarError,
    traits::{Run, Uninstallable},
};

#[derive(Parser, Clone)]
pub struct Uninstall {
    /// The name of the tool to remove. If none is provided, defaults to all tools.
    #[command(subcommand)]
    component: UninstallableComponent,
}

impl Run for Uninstall {
    fn run(&self) -> Result<(), SolarError> {
        self.component.uninstall(&working_dir()?)
    }
}
