use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::github_workflows::{installation::Installation, workflows_path},
    solar_error::SolarError,
    traits::{GetPartialInstall, Uninstallable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct GithubWorkflowsUninstaller {
    // List of workflow file names to remove specifically, rather than the entire directory.
    #[arg(short, long, num_args = 0..)]
    file_names: Option<Vec<String>>,
}

impl Uninstallable for GithubWorkflowsUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        let workflows_path = workflows_path(path);

        // If file names are given, remove only the specified file_names
        if let Some(file_names) = &self.file_names {
            // Get current installation if it exists.
            let current_installation: Installation = Installation::get_current(path)?;

            // If the GitHub workflows directory exists, and the file being removed exists, remove it.
            if let Some(workflow_files) = current_installation.workflow_files() {
                for file_name in file_names {
                    if workflow_files.contains(file_name) {
                        fs::remove_file(workflows_path.join(file_name))?;
                    }
                }
            }

            // If the GitHub workflows directory is now empty, attempt removal. Ignore error.
            let _ = fs::remove_dir(&workflows_path);
        } else {
            // If no file names are given, remove entire github workflows directory.
            fs::remove_dir_all(&workflows_path)?;
        }

        Ok(())
    }
}
