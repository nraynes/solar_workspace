use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    components::vhooks::{installation::Installation, move_hooks::move_hooks},
    solar_error::SolarError,
    tools::git::{DEFAULT_GIT_HOOKS_DIR, GitRepository},
    traits::Uninstallable,
};
use clap::Parser;
use derive_getters::Getters;
use derive_new::new;
use rust_terminal::Terminal;

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct VhooksUninstaller {
    /// Remove all git hooks rather than putting them in the default unversioned git hooks directory.
    #[arg(short, long, default_value = "false")]
    remove_hooks: bool,
}

impl Uninstallable for VhooksUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;
        let default_hooks_path = PathBuf::from(DEFAULT_GIT_HOOKS_DIR);

        // If default git hooks path is being used, hooks are not versioned and no action is required.
        if !git_repository.installation().hooks_path().default() {
            // Default git hooks folder must exist.
            fs::create_dir_all(&default_hooks_path)?;

            // If not removing hooks, move them to the default hooks directory.
            if !self.remove_hooks {
                move_hooks(
                    git_repository.installation().hooks_path().path(),
                    &default_hooks_path,
                )?;
            }

            // Remove the versioned hooks folder.
            fs::remove_dir_all(git_repository.installation().hooks_path().path())?;

            // Set the hooks directory.
            Terminal::command().current_dir(path).piped().run(
                "git",
                vec![
                    "config",
                    "core.hooksPath",
                    default_hooks_path
                        .to_str()
                        .ok_or("Could not convert default git hooks path to string.")?,
                ],
            )?;
        }

        Ok(())
    }
}
