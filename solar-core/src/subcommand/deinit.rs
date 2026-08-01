// use std::path::{Path, PathBuf};

// use clap::Parser;

// use crate::{Action, Config, SolarError, ToolTrait};

// #[derive(Parser, Clone)]
// pub struct Deinit {
//     /// The destination to deinitialize the project.
//     #[arg(short, long, default_value = ".")]
//     destination: PathBuf,
// }

// impl Deinit {
//     pub fn run(&mut self) -> Result<(), SolarError> {
//         solar_deinit(
//             &mut Config::load_from(&self.destination)?,
//             &self.destination,
//         )
//     }
// }

// pub fn solar_deinit(config: &mut Config, destination: &Path) -> Result<(), SolarError> {
//     config.act(&Action::UNINSTALL, Some(destination))
// }
