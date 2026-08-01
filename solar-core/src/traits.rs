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
}

pub trait GetPartialInstall {
    fn get_current(path: &Path) -> Result<Self, SolarError>
    where
        Self: Sized;
}
