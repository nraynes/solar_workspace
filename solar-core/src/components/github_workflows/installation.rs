use std::{fs, path::Path};

use derive_getters::Getters;

use crate::{
    components::github_workflows::workflows_path, solar_error::SolarError,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    project_name: String,
    workflow_files: Option<Vec<String>>,
}

impl Installation {
    pub fn read_workflows(path: &Path) -> Option<Vec<String>> {
        let mut workflow_files = Vec::new();
        if let Ok(read_dir) = fs::read_dir(path) {
            for dir_entry_result in read_dir {
                if let Ok(dir_entry) = dir_entry_result
                    && dir_entry.path().is_file()
                {
                    workflow_files.push(dir_entry.path().file_name()?.to_str()?.to_string());
                }
            }
            return Some(workflow_files);
        }
        None
    }
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        Ok(Self {
            project_name: path
                .canonicalize()?
                .file_name()
                .ok_or("Could not get name of working directory")?
                .to_str()
                .ok_or("Could not convert directory name to string.")?
                .to_string(),
            workflow_files: Self::read_workflows(&workflows_path(path)),
        })
    }
}
