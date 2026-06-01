mod bin_release;
mod lib_release;

pub use bin_release::BinRelease;
pub use lib_release::LibRelease;

use crate::tool::github_workflows::WorkflowTrait;

pub const FILE_NAME: &str = "release.yml";

pub enum ReleaseWf {
    Bin(BinRelease),
    Lib(LibRelease),
}

impl WorkflowTrait for ReleaseWf {
    fn get(&self) -> String {
        match self {
            Self::Bin(v) => v.get(),
            Self::Lib(v) => v.get(),
        }
    }
}
