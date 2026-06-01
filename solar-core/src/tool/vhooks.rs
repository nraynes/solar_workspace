use crate::{Config, Global, SOLARCONFIGNAME, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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
}

impl ToolTrait for Vhooks {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
        // Update configuration file.
        let config =
            match Config::load_from_file(self.destination.join(PathBuf::from(SOLARCONFIGNAME))) {
                Ok(cfg) => cfg.set_vhooks(Some(self.clone())),
                Err(_) => Config::new(Some(self.clone()), None, None, None, None, None, None),
            };
        config.save_to_file(self.destination.join(PathBuf::from(SOLARCONFIGNAME)))?;

        // Ensure working directory is a git repository.
        Global::git_init(&self.destination)?;

        // Path to the versioned hooks directory.
        let hooks_path = self.destination.join(PathBuf::from(&self.name));

        // Create the new hooks directory.
        fs::create_dir_all(&hooks_path)?;

        // Set the new hooks directory as the git hooks directory.
        Terminal::command()
            .current_dir(self.destination.clone())
            .piped()
            .run(
                "git",
                vec![
                    "config",
                    "core.hooksPath",
                    PathBuf::from(format!("./{}", &self.name))
                        .to_str()
                        .ok_or("Could not convert path to string.")?,
                ],
            )?;
        Ok(())
    }

    fn upgrade(&self) -> Result<(), SolarError> {
        println!("Upgrade does not apply to vhooks - nothing to upgrade.");
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        let config = Config::load_from_file(self.destination.join(PathBuf::from(SOLARCONFIGNAME)))?;
        let vhooks: Self = config
            .vhooks()
            .clone()
            .ok_or("Cannot uninstall vhooks - vhooks not found in configuration.")?;

        // Paths to the versioned hooks directory and default directory.
        let hooks_path = self.destination.join(vhooks.name());
        let default_path = self.destination.join(Global::default_git_hook_dir());

        // Must be a git repository in order to set default hook directory.
        Global::is_git(&self.destination)?;

        // Git hooks folder must exist.
        fs::create_dir_all(&default_path)?;

        // If not removing hooks, move them to the default hooks directory.
        if !self.remove_all {
            for item in fs::read_dir(&hooks_path)? {
                let item = item?;
                fs::rename(item.path(), &default_path.join(item.file_name()))?;
            }
        }

        // Set the new hooks directory as the default git hooks directory.
        Terminal::command()
            .current_dir(self.destination.clone())
            .piped()
            .run(
                "git",
                vec![
                    "config",
                    "core.hooksPath",
                    PathBuf::from(format!(
                        "./{}",
                        Global::default_git_hook_dir()
                            .to_str()
                            .ok_or("Could not convert path to string.")?
                    ))
                    .to_str()
                    .ok_or("Could not convert path to string.")?,
                ],
            )?;

        // Remove the versioned hooks folder.
        fs::remove_dir_all(&hooks_path)?;

        // Update configuration.
        let config = config.set_vhooks(None);
        match config.is_empty() {
            true => fs::remove_file(self.destination.join(PathBuf::from(SOLARCONFIGNAME)))?,
            false => config.save_to_file(self.destination.join(PathBuf::from(SOLARCONFIGNAME)))?,
        }

        Ok(())
    }
}
