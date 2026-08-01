use cargo_solar::Args;
use clap::Parser;
use solar_core::{solar_error::SolarError, traits::Run};

fn main() -> Result<(), SolarError> {
    let args = Args::parse();
    args.unitcommand.run()
}
