use cargo_solar::Args;
use clap::Parser;
use semver_common::Alert;
use solar_core::Subcommand;

fn main() -> Result<(), Alert> {
    let args = Args::parse();
    match args.subcommand {
        Subcommand::INIT(cmd) => cmd.run(),
        Subcommand::NEW(cmd) => cmd.run(),
        Subcommand::UPGRADE(cmd) => cmd.run(),
        Subcommand::INSTALL(cmd) => cmd.run(),
        Subcommand::REMOVE(cmd) => cmd.run(),
    }
}
