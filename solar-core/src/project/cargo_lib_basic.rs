use std::path::Path;

use clap::Parser;

use crate::{solar_error::SolarError, traits::ConfigureProject};

#[derive(Parser, Clone)]
pub struct CargoLibBasic {}

impl ConfigureProject for CargoLibBasic {
    fn deinit(&self, path: &Path) -> Result<(), SolarError> {
        Ok(())
    }

    fn init(&self, path: &Path) -> Result<(), SolarError> {
        Ok(())
    }

    fn update(&self, path: &Path) -> Result<(), SolarError> {
        Ok(())
    }
}
