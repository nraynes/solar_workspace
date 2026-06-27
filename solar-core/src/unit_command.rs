use clap::{Parser, Subcommand as SC};

use crate::Subcommand;

#[derive(SC, Clone)]
pub enum UnitCommand {
    SOLAR(SolarCommand),
}

#[derive(Parser, Clone)]
pub struct SolarCommand {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}
