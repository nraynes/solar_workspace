use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;
use rust_terminal::Terminal;

use crate::{
    components::vhooks::{installation::Installation, move_hooks::move_hooks},
    tools::git::GitRepository,
    solar_error::SolarError,
    traits::Installable,
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct VhooksInstaller {
    /// Path to versioned git hooks directory.
    #[arg(long, default_value = ".hooks")]
    hooks_path: PathBuf,
}

impl Installable for VhooksInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // Path to the versioned hooks directory.
        let hooks_path_str = self
            .hooks_path
            .to_str()
            .ok_or("Could not convert hooks_path to &str.")?;

        // Create the new hooks directory.
        fs::create_dir_all(&self.hooks_path)?;

        // Move any hooks if the hooks path has changed.
        let old_hooks_path = git_repository.installation().hooks_path().path();
        if old_hooks_path != &self.hooks_path {
            move_hooks(old_hooks_path, &self.hooks_path)?;

            // Remove old hooks directory if it is not default.
            if !git_repository.installation().hooks_path().default() {
                fs::remove_dir_all(old_hooks_path)?;
            }
        }

        // Set the new hooks directory.
        Terminal::command()
            .current_dir(path)
            .piped()
            .run("git", vec!["config", "core.hooksPath", hooks_path_str])?;

        Ok(())
    }
}
