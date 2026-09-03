use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::github_workflows::{
        WORKFLOW_ALREADY_EXISTS_ERROR_MESSAGE, installation::Installation, workflow::Workflow,
        workflows_path,
    },
    solar_error::SolarError,
    traits::{GetPartialInstall, Installable},
};

#[derive(Parser, Clone, PartialEq, Debug, Getters, new)]
pub struct GithubWorkflowsInstaller {
    /// The workflow preset to install.
    #[command(subcommand)]
    workflow: Workflow,

    /// The name of the resulting workflow yaml file.
    #[arg(short, long)]
    file_name: String,
}

impl Installable for GithubWorkflowsInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let workflows_dir_path = workflows_path(path);

        // Ensure github workspace folders exist.
        if current_installation.workflow_files().is_none() {
            fs::create_dir_all(&workflows_dir_path)?;
        }

        // Create the workflow if it doesn't already exist.
        let workflow_path = workflows_dir_path.join(&self.file_name);
        if current_installation
            .workflow_files()
            .as_ref()
            .is_none_or(|workflow_files| !workflow_files.contains(&self.file_name))
        {
            let mut workflow_file = File::options()
                .create_new(true)
                .write(true)
                .open(&workflow_path)?;
            workflow_file.write_all(
                yaml_serde::to_string(&self.workflow.build_yaml(&current_installation))?.as_bytes(),
            )?;
        } else {
            println!("{}", WORKFLOW_ALREADY_EXISTS_ERROR_MESSAGE);
        }

        Ok(())
    }
}
