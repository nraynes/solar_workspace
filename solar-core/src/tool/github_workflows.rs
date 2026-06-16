mod parameters;
mod workflow;
mod workflow_file;

use derive_getters::Getters;
pub use parameters::Parameters;
use serde::{Deserialize, Serialize};
pub use workflow::Workflow;

use crate::{Config, SolarError, ToolTrait, tool::github_workflows::workflow_file::WorkflowFile};
use clap::Parser;
use std::{
    collections::HashMap,
    fs::{self, DirEntry, File},
    hash::Hash,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct GithubWorkflows {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// Use the release workflow in this project.
    #[arg(short, long, num_args = 0..)]
    workflows_list: Option<Vec<Workflow>>,
}

impl GithubWorkflows {
    pub fn new(destination: PathBuf, workflows_list: Option<Vec<Workflow>>) -> Self {
        Self {
            destination,
            workflows_list,
        }
    }

    fn workflows_path(&self) -> PathBuf {
        self.destination.join(".github/workflows")
    }

    fn get_current_workflows(&self) -> Result<Vec<DirEntry>, SolarError> {
        let workflows_dir = self.workflows_path();
        let mut workflow_entries = Vec::new();
        for entry in fs::read_dir(workflows_dir)? {
            workflow_entries.push(entry?);
        }
        Ok(workflow_entries)
    }

    fn workflow_list_to_map<'a>(
        &self,
        workflows_list: &'a Vec<Workflow>,
    ) -> Result<HashMap<WorkflowFile, &'a Workflow>, SolarError> {
        let mut workflow_map = HashMap::new();
        for workflow in workflows_list {
            let workflow_type = workflow.get().file();
            if workflow_map.get(&workflow_type).is_none() {
                workflow_map.insert(workflow_type, workflow);
            }
        }
        Ok(workflow_map)
    }

    fn extend_no_overwrite<K, V>(
        &self,
        mut hash_map_one: HashMap<K, V>,
        hash_map_two: HashMap<K, V>,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        for (key, value) in hash_map_two {
            if hash_map_one.get(&key).is_none() {
                hash_map_one.insert(key, value);
            }
        }
        hash_map_one
    }

    fn get_parameters(&self) -> Result<Parameters, SolarError> {
        Ok(Parameters::new(
            self.destination
                .canonicalize()?
                .file_name()
                .ok_or("Could not get name of working directory")?
                .to_str()
                .ok_or("Could not convert directory name to string.")?
                .to_string(),
        ))
    }
}

impl ToolTrait for GithubWorkflows {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let workflows_dir = self.workflows_path();
        let parameters = self.get_parameters()?;
        let workflows_list = self
            .workflows_list
            .as_ref()
            .ok_or("At least one workflow must be given for installation.")?;

        let mut workflows_map = self.workflow_list_to_map(workflows_list)?;

        // Ensure github workspace folders exist.
        fs::create_dir_all(&workflows_dir)?;

        // Get or create configuration.
        let config = Config::load_or_default(&self.destination);
        if let Some(workflows_config) = config.github_workflows()
            && let Some(config_workflows) = workflows_config.workflows_list()
        {
            let config_workflows_map = self.workflow_list_to_map(config_workflows)?;
            workflows_map = self.extend_no_overwrite(workflows_map, config_workflows_map);
        }

        // Create the workflows.
        for workflow in workflows_map.values() {
            let workflow_type = workflow.get();
            let workflow_path = workflows_dir.join(workflow_type.file().name());
            if !fs::exists(&workflow_path)? {
                File::create(&workflow_path)?;
            }
            let mut workflow_file = File::options()
                .write(true)
                .truncate(true)
                .open(&workflow_path)?;
            workflow_file.write_all(workflow_type.get(&parameters).as_bytes())?;
        }

        // Save configuration.
        self.workflows_list = Some(Vec::from_iter(
            workflows_map.values().map(|v| v.to_owned().to_owned()),
        ));
        config.set_github_workflows(Some(self.clone())).save()?;

        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        // Get configuration
        let config = Config::load_from(&self.destination)?;
        let current_config = config.github_workflows().as_ref().ok_or(
            "Cannot uninstall github_workflows - github_workflows not found in configuration.",
        )?;

        // Remove the workflows from the configuration.
        let workflows_dir = self.workflows_path();
        if let Some(workflows_list) = current_config.workflows_list() {
            for workflow in workflows_list {
                let workflow_path = workflows_dir.join(workflow.get().file().name());
                if fs::exists(&workflow_path)? {
                    fs::remove_file(workflow_path)?;
                }
            }
        }

        // If workflows directory is empty, remove it.
        if self.get_current_workflows()?.is_empty() {
            fs::remove_dir(workflows_dir)?;
        }

        // Update configuration, remove if empty.
        let config = config.set_github_workflows(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
