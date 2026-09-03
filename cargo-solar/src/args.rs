use clap::Parser;
use solar_core::unit_command::UnitCommand;

#[derive(Parser)]
pub struct Args {
    /// The specific solar command to use.
    #[command(subcommand)]
    pub unitcommand: UnitCommand,
}
