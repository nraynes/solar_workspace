mod cargo_bin_basic;
mod cargo_lib_basic;
mod cargo_proc_macro_basic;

pub use cargo_bin_basic::CargoBinBasic;
pub use cargo_lib_basic::CargoLibBasic;
pub use cargo_proc_macro_basic::CargoProcMacroBasic;

use std::path::Path;

use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use crate::{solar_error::SolarError, traits::ConfigureProject};

#[enum_dispatch(ConfigureProject)]
#[derive(Subcommand, Clone)]
pub enum Project {
    CargoBinBasic(CargoBinBasic),
    CargoLibBasic(CargoLibBasic),
    CargoProcMacroBasic(CargoProcMacroBasic),
}
