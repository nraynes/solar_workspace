use clap::Parser;

use crate::{
    components::UpgradableComponent,
    solar_error::SolarError,
    traits::{Run, Upgradable},
    working_dir,
};

#[derive(Parser, Clone)]
pub struct Upgrade {
    /// The name of the component to upgrade.
    #[command(subcommand)]
    component: UpgradableComponent,
}

impl Run for Upgrade {
    fn run(&self) -> Result<(), SolarError> {
        self.component.upgrade(&working_dir()?)
    }
}
