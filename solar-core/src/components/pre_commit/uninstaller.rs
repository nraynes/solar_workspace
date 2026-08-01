use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::{
        git::GitRepository,
        pre_commit::{PRE_COMMIT, installation::Installation},
    },
    solar_error::SolarError,
    traits::Uninstallable,
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct PreCommitUninstaller {}

impl Uninstallable for PreCommitUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // Remove pre-commit script if it exists.
        if *git_repository.installation().script_exists() {
            fs::remove_file(
                git_repository
                    .installation()
                    .hooks_path()
                    .path()
                    .join(PRE_COMMIT),
            )?;
        }

        Ok(())
    }
}
