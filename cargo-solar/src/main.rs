use cargo_solar::Args;
use clap::Parser;
use solar_core::{SolarError, Subcommand};

fn main() -> Result<(), SolarError> {
    let args = Args::parse();
    match args.subcommand {
        Subcommand::INIT(cmd) => cmd.run(),
        Subcommand::NEW(cmd) => cmd.run(),
        Subcommand::UPGRADE(cmd) => cmd.run(),
        Subcommand::INSTALL(cmd) => cmd.run(),
        Subcommand::UNINSTALL(cmd) => cmd.run(),
    }
}
