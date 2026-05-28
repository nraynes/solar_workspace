mod release;
mod test;
mod workflow_trait;

use release::{BinRelease, LibRelease, ReleaseWf};
use serde::{Deserialize, Serialize};
use test::{GeneralTest, TestWf};

use crate::{
    Global, SolarError, ToolTrait, tool::github_workflows::workflow_trait::HasConstructor,
};
use clap::{Parser, ValueEnum};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
pub use workflow_trait::WorkflowTrait;

#[derive(ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ReleaseWfType {
    BIN,
    LIB,
}

impl ReleaseWfType {
    fn workflow(&self) -> ReleaseWf {
        match self {
            Self::BIN => ReleaseWf::BIN(BinRelease::new()),
            Self::LIB => ReleaseWf::LIB(LibRelease::new()),
        }
    }
}

impl Default for ReleaseWfType {
    fn default() -> Self {
        Self::BIN
    }
}

#[derive(ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TestWfType {
    GENERAL,
}

impl TestWfType {
    fn workflow(&self) -> TestWf {
        match self {
            Self::GENERAL => TestWf::GENERAL(GeneralTest::new()),
        }
    }
}

impl Default for TestWfType {
    fn default() -> Self {
        Self::GENERAL
    }
}

fn default_release_wf() -> Option<ReleaseWfType> {
    Some(ReleaseWfType::BIN)
}

fn default_test_wf() -> Option<TestWfType> {
    Some(TestWfType::GENERAL)
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Workflows {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(default = "Global::default_destination")]
    destination: PathBuf,

    /// Use the release workflow in this project.
    #[arg(short, long, default_value = "bin")]
    #[serde(default = "default_release_wf")]
    release_workflow: Option<ReleaseWfType>,

    /// Use the test workflow in this project.
    #[arg(short, long, default_value = "general")]
    #[serde(default = "default_test_wf")]
    test_workflow: Option<TestWfType>,
}

impl Workflows {
    pub fn new(
        destination: PathBuf,
        release_workflow: Option<ReleaseWfType>,
        test_workflow: Option<TestWfType>,
    ) -> Self {
        Self {
            destination,
            release_workflow,
            test_workflow,
        }
    }

    fn workflows_path(&self) -> PathBuf {
        self.destination.join(PathBuf::from(".github/workflows"))
    }
}

impl ToolTrait for Workflows {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
        // Ensure github workspace folders exist.
        let workflows_dir = self.workflows_path();
        fs::create_dir_all(&workflows_dir)?;

        // Create the release workflow.
        if let Some(workflow_type) = &self.release_workflow {
            let mut workflow_file = File::create(workflows_dir.join(release::FILE_NAME))?;
            let mut workflow_obj = workflow_type.workflow();
            if let ReleaseWf::BIN(bin_release) = &mut workflow_obj {
                bin_release.set_project_name(
                    self.destination
                        .file_name()
                        .ok_or("Could not get name of working directory")?
                        .to_str()
                        .ok_or("Could not convert directory name to string.")?,
                );
            }
            workflow_file.write_all(workflow_obj.get().as_bytes())?;
        }

        // Create the test workflow.
        if let Some(workflow_type) = &self.test_workflow {
            let mut workflow_file = File::create(workflows_dir.join(test::FILE_NAME))?;
            workflow_file.write_all(workflow_type.workflow().get().as_bytes())?;
        }
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        let workflows_dir = self.workflows_path();
        let release_file = workflows_dir.join(release::FILE_NAME);
        let test_file = workflows_dir.join(test::FILE_NAME);
        if fs::exists(&release_file)? {
            fs::remove_file(release_file)?;
        }
        if fs::exists(&test_file)? {
            fs::remove_file(test_file)?;
        }
        Ok(())
    }
}
