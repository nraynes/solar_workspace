mod cargo_bin_basic;
mod cargo_lib_basic;
mod cargo_proc_macro_basic;
mod cargo_workspace_basic;
mod cargo_bin_workspaced;
mod cargo_lib_workspaced;
mod cargo_proc_macro_workspaced;

pub use cargo_bin_basic::CargoBinBasic;
pub use cargo_lib_basic::CargoLibBasic;
pub use cargo_proc_macro_basic::CargoProcMacroBasic;
pub use cargo_workspace_basic::CargoWorkspaceBasic;
pub use cargo_bin_workspaced::CargoBinWorkspaced;
pub use cargo_lib_workspaced::CargoLibWorkspaced;
pub use cargo_proc_macro_workspaced::CargoProcMacroWorkspaced;

use clap::Subcommand;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(ConfigureProject)]
#[derive(Subcommand, Clone)]
pub enum Project {
    CargoBinBasic(CargoBinBasic),
    CargoLibBasic(CargoLibBasic),
    CargoProcMacroBasic(CargoProcMacroBasic),
    CargoWorkspaceBasic(CargoWorkspaceBasic),
}
