mod release;
mod test;

use release::{BIN_RELEASE, LIB_RELEASE};
use test::GENERAL_TEST;

use crate::{SolarError, ToolTrait};
use clap::{Parser, ValueEnum};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(ValueEnum, Clone, PartialEq, Debug)]
enum ReleaseWfType {
    BIN,
    LIB,
}

impl ReleaseWfType {
    fn get_workflow(&self) -> String {
        match self {
            Self::BIN => String::from(BIN_RELEASE),
            Self::LIB => String::from(LIB_RELEASE),
        }
    }
}

impl Default for ReleaseWfType {
    fn default() -> Self {
        Self::BIN
    }
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
enum TestWfType {
    GENERAL,
}

impl TestWfType {
    fn get_workflow(&self) -> String {
        match self {
            Self::GENERAL => String::from(GENERAL_TEST),
        }
    }
}

impl Default for TestWfType {
    fn default() -> Self {
        Self::GENERAL
    }
}

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Workflows {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// Use the release workflow in this project.
    #[arg(short, long, default_value = "bin")]
    release_workflow: Option<ReleaseWfType>,

    /// Use the test workflow in this project.
    #[arg(short, long, default_value = "general")]
    test_workflow: Option<TestWfType>,
}

impl Workflows {
    fn workflows_path(&self) -> PathBuf {
        self.working_dir.join(PathBuf::from(".github/workflows"))
    }
}

impl ToolTrait for Workflows {
    fn install(&self) -> Result<(), SolarError> {
        // Ensure github workspace folders exist.
        let workflows_dir = self.workflows_path();
        fs::create_dir_all(&workflows_dir)?;

        // Create the release workflow.
        if let Some(workflow_type) = &self.release_workflow {
            let mut workflow_file = File::create(workflows_dir.join(release::FILE_NAME))?;
            workflow_file.write_all(workflow_type.get_workflow().as_bytes())?;
        }

        // Create the test workflow.
        if let Some(workflow_type) = &self.test_workflow {
            let mut workflow_file = File::create(workflows_dir.join(test::FILE_NAME))?;
            workflow_file.write_all(workflow_type.get_workflow().as_bytes())?;
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
