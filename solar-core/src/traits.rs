use std::path::Path;

use enum_dispatch::enum_dispatch;

use crate::{components::*, project::*, solar_error::SolarError, subcommand::*};

#[enum_dispatch]
pub trait Run {
    fn run(&self) -> Result<(), SolarError>;
}

#[enum_dispatch]
pub trait Installable {
    fn install(&self, path: &Path) -> Result<(), SolarError>;
}

#[enum_dispatch]
pub trait Uninstallable {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError>;
}

#[enum_dispatch]
pub trait Upgradable {
    fn upgrade(&self, path: &Path) -> Result<(), SolarError>;
}

#[enum_dispatch]
pub trait ConfigureProject {
    fn init(&self, path: &Path) -> Result<(), SolarError>;

    fn deinit(&self, path: &Path) -> Result<(), SolarError>;

    fn update(&self, path: &Path) -> Result<(), SolarError>;

    fn clean_up_on_error(
        &self,
        path: &Path,
        result: Result<(), SolarError>,
    ) -> Result<(), SolarError> {
        if let Err(e) = result {
            self.deinit(path)?;
            return Err(e);
        }
        Ok(())
    }

    fn combine_errors(&self, results: &[Result<(), SolarError>]) -> Result<(), SolarError> {
        let mut combined_error = String::new();
        let mut error_occurred = false;
        for result in results {
            if let Err(e) = result {
                combined_error += e.to_string().as_str();
                error_occurred = true;
            }
        }
        match error_occurred {
            true => Err(SolarError::from(combined_error)),
            false => Ok(()),
        }
    }
}

pub trait GetPartialInstall {
    fn get_current(path: &Path) -> Result<Self, SolarError>
    where
        Self: Sized;
}
