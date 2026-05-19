mod general_test;

pub use general_test::GeneralTest;

use crate::tool::github_workflows::WorkflowTrait;

pub const FILE_NAME: &str = "test.yml";

pub enum TestWf {
    GENERAL(GeneralTest),
}

impl WorkflowTrait for TestWf {
    fn get(&self) -> String {
        match self {
            Self::GENERAL(v) => v.get(),
        }
    }
}
