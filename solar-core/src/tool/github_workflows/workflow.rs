mod release_cargo_bin_general;
mod release_cargo_lib_general;
mod test_cargo_general;

pub use release_cargo_bin_general::ReleaseCargoBinGeneral;
pub use release_cargo_lib_general::ReleaseCargoLibGeneral;
use serde::{Deserialize, Serialize};
pub use test_cargo_general::TestCargoGeneral;

use clap::ValueEnum;

use crate::tool::github_workflows::{parameters::Parameters, workflow_file::WorkflowFile};

#[derive(
    ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Default, Eq, PartialOrd, Ord,
)]
pub enum Workflow {
    #[default]
    ReleaseCargoBinGeneral,
    ReleaseCargoLibGeneral,
    TestCargoGeneral,
}

impl Workflow {
    pub fn get(&self) -> Box<dyn WorkflowDetails> {
        match self {
            Self::ReleaseCargoBinGeneral => Box::new(ReleaseCargoBinGeneral::new()),
            Self::ReleaseCargoLibGeneral => Box::new(ReleaseCargoLibGeneral::new()),
            Self::TestCargoGeneral => Box::new(TestCargoGeneral::new()),
        }
    }
}

pub trait WorkflowDetails {
    fn new() -> Self
    where
        Self: Sized;

    fn file(&self) -> WorkflowFile;

    fn get(&self, parameters: &Parameters) -> String;
}
