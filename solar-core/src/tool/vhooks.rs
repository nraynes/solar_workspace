use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

fn default_name() -> String {
    ".hooks".to_string()
}

fn default_false() -> bool {
    false
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Vhooks {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(default = "Global::default_destination")]
    destination: PathBuf,

    /// Git hooks directory name.
    #[arg(short, long, default_value = ".hooks")]
    #[serde(default = "default_name")]
    name: String,

    /// Whether to remove all hooks when removing vhooks, or just put thim in unversioned git hooks directory.
    #[arg(short, long, default_value = "false")]
    #[serde(default = "default_false")]
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

    fn pathbuf_to_str(&self, path: PathBuf) -> Result<String, SolarError> {
        Ok(String::from(
            path.to_str()
                .ok_or("Could not convert path buffer to string")?,
        ))
    }
}

impl ToolTrait for Vhooks {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
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
                    &hooks_path
                        .to_str()
                        .ok_or("Could not convert path to string.")?,
                ],
            )?;
        Ok(())
    }

    fn upgrade(&self) -> Result<(), SolarError> {
        self.install()?;
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Paths to the versioned hooks directory and default directory.
        let hooks_path = self.pathbuf_to_str(self.destination.join(&self.name).join("*"))?;
        let default_path = self.pathbuf_to_str(Global::default_git_hook_dir())?;

        // Must be a git repository in order to set default hook directory.
        Global::git_init(&self.destination)?;

        // Git hooks folder must exist.
        Terminal::command()
            .current_dir(self.destination.clone())
            .piped()
            .run("mkdir", ["-p", &default_path])?;

        // If not removing hooks, move them to the default hooks directory.
        if !self.remove_all {
            Terminal::command()
                .current_dir(self.destination.clone())
                .piped()
                .run("mv", [&hooks_path, &default_path])?;
        }

        // Set the new hooks directory as the default git hooks directory.
        Terminal::command()
            .current_dir(self.destination.clone())
            .piped()
            .run("git", vec!["config", "core.hooksPath", &default_path])?;

        // Remove the versioned hooks folder.
        Terminal::command()
            .current_dir(self.destination.clone())
            .piped()
            .run("rm", ["-rf", &hooks_path])?;
        Ok(())
    }
}
