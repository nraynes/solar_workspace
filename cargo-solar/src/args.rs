use clap::Parser;

use solar_core::Subcommand;

#[derive(Parser)]
pub struct Args {
    /// The specific solar command to use.
    #[command(subcommand)]
    pub subcommand: Subcommand,
}
