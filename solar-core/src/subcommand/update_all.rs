use clap::Parser;
use strum::IntoEnumIterator;

use crate::{
    components::UpgradableComponent,
    solar_error::SolarError,
    traits::{Run, Upgradable},
    working_dir,
};

#[derive(Parser, Clone)]
pub struct UpdateAll {}

impl Run for UpdateAll {
    fn run(&self) -> Result<(), SolarError> {
        let path = working_dir()?;

        for component in UpgradableComponent::iter() {
            let _ = component.upgrade(&path);
        }

        Ok(())
    }
}
