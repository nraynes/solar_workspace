use clap::Subcommand;
use yaml_serde::Value;

use crate::components::github_workflows::installation::Installation;

mod cargo_any_general_test;
mod cargo_bin_general_release;
mod cargo_lib_general_release;

pub use cargo_any_general_test::CargoAnyGeneralTest;
pub use cargo_bin_general_release::CargoBinGeneralRelease;
pub use cargo_lib_general_release::CargoLibGeneralRelease;

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Workflow {
    CargoBinGeneralRelease(CargoBinGeneralRelease),
    CargoLibGeneralRelease(CargoLibGeneralRelease),
    CargoAnyGeneralTest(CargoAnyGeneralTest),
}

impl Workflow {
    pub fn build_yaml(&self, installation: &Installation) -> Value {
        match self {
            Self::CargoBinGeneralRelease(cargo_bin_general_release) => {
                cargo_bin_general_release.build_yaml(installation.project_name())
            }
            Self::CargoLibGeneralRelease(cargo_lib_general_release) => {
                cargo_lib_general_release.build_yaml()
            }
            Self::CargoAnyGeneralTest(cargo_any_general_test) => {
                cargo_any_general_test.build_yaml()
            }
        }
    }
}
