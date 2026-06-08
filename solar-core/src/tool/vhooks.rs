use crate::{Config, Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn default_name() -> String {
    ".hooks".to_string()
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct Vhooks {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// Git hooks directory name.
    #[arg(short, long, default_value = ".hooks")]
    #[serde(default = "default_name")]
    name: String,

    /// Whether to remove all hooks when removing vhooks, or just put thim in unversioned git hooks directory.
    #[arg(short, long, default_value = "false")]
    #[serde(skip)]
    remove_all: bool,
}

impl Vhooks {
    pub fn new(destination: PathBuf, name: String, remove_all: bool) -> Self {
        Self {
            destination,
            name,
            remove_all,
        }
    }

    pub fn move_hooks(prev: &PathBuf, new: &Path) -> Result<(), SolarError> {
        for item in fs::read_dir(prev)? {
            let item = item?;
            fs::rename(item.path(), new.join(item.file_name()))?;
        }
        Ok(())
    }

    pub fn set_hooks_path(&self, path: PathBuf) -> Result<(), SolarError> {
        Terminal::command()
            .current_dir(&self.destination)
            .piped()
            .run(
                "git",
                vec![
                    "config",
                    "core.hooksPath",
                    path.to_str().ok_or("Could not convert path to string.")?,
                ],
            )?;
        Ok(())
    }
}

impl ToolTrait for Vhooks {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        // Update configuration file.
        let config = Config::load_or_default(&self.destination);
        let current_tool_cfg = config.vhooks().clone();
        config.set_vhooks(Some(self.clone())).save()?;

        // Ensure working directory is a git repository.
        Global::git_init(&self.destination)?;

        // Path to the versioned hooks directory.
        let hooks_path = self.destination.join(&self.name);

        // Create the new hooks directory.
        fs::create_dir_all(&hooks_path)?;

        // Set the new hooks directory as the git hooks directory.
        self.set_hooks_path(PathBuf::from(format!("./{}", &self.name)))?;

        // If there is a current installation, move the hooks from the old directory to the new one.
        if let Some(vhooks) = current_tool_cfg {
            let old_hooks_path = self.destination.join(vhooks.name());
            Self::move_hooks(&old_hooks_path, &hooks_path)?;
            fs::remove_dir_all(&old_hooks_path)?;
        }

        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        let config = Config::load_from(&self.destination)?;
        let current_tool_cfg: Self = config
            .vhooks()
            .clone()
            .ok_or("Cannot uninstall vhooks - vhooks not found in configuration.")?;

        // Paths to the versioned hooks directory and default directory.
        let hooks_path = self.destination.join(current_tool_cfg.name());
        let default_path = self.destination.join(Global::default_git_hook_dir());

        // Must be a git repository in order to set default hook directory.
        Global::is_git(&self.destination)?;

        // Git hooks folder must exist.
        fs::create_dir_all(&default_path)?;

        // If not removing hooks, move them to the default hooks directory.
        if !self.remove_all {
            Self::move_hooks(&hooks_path, &default_path)?;
        }

        // Set the new hooks directory as the default git hooks directory.
        self.set_hooks_path(PathBuf::from(format!(
            "./{}",
            Global::default_git_hook_dir()
                .to_str()
                .ok_or("Could not convert path to string.")?
        )))?;

        // Remove the versioned hooks folder.
        fs::remove_dir_all(&hooks_path)?;

        // Update configuration, remove if empty.
        let config = config.set_vhooks(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
