use clap::{Parser, Subcommand as SC};
use enum_dispatch::enum_dispatch;

use crate::{solar_error::SolarError, subcommand::Subcommand, traits::Run};

#[enum_dispatch(Run)]
#[derive(SC, Clone)]
pub enum UnitCommand {
    SOLAR(SolarCommand),
}

#[derive(Parser, Clone)]
pub struct SolarCommand {
    #[command(subcommand)]
    subcommand: Subcommand,
}

impl Run for SolarCommand {
    fn run(&self) -> Result<(), SolarError> {
        self.subcommand.run()
    }
}
