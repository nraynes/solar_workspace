use cargo_solar::Args;
use clap::Parser;
use solar_core::{SolarError, Subcommand};

fn main() -> Result<(), SolarError> {
    let args = Args::parse();
    match args.subcommand {
        Subcommand::INIT(mut cmd) => cmd.run(),
        Subcommand::NEW(mut cmd) => cmd.run(),
        Subcommand::UPDATE(mut cmd) => cmd.run(),
        Subcommand::UPGRADE(mut cmd) => cmd.run(),
        Subcommand::INSTALL(mut cmd) => cmd.run(),
        Subcommand::UNINSTALL(mut cmd) => cmd.run(),
        Subcommand::DEINIT(mut cmd) => cmd.run(),
    }
}
