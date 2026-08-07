use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::commitalyzer::{
        COMMIT_MSG_NAME, download::download_commitalyzer_binary, installation::Installation,
    },
    tools::git::GitRepository,
    solar_error::SolarError,
    traits::Upgradable,
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct CommitalyzerUpgrader {}

impl Upgradable for CommitalyzerUpgrader {
    fn upgrade(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // Set path variables.
        let bin_path = git_repository
            .installation()
            .hooks_path()
            .path()
            .join(COMMIT_MSG_NAME);

        // Remove and install latest binary.
        if *git_repository.installation().hook_bin()
            && fs::remove_file(&bin_path).is_ok()
            && let Err(e) = download_commitalyzer_binary(&bin_path)
        {
            println!(
                "There was an error while trying to upgrade commitalyzer binary.\n\nERROR: {}",
                e
            );
        };

        Ok(())
    }
}
