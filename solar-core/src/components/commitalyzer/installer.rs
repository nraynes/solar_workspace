use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::commitalyzer::{
        COMMIT_RULES_NAME,
        download::{download_commitalyzer_binary, download_commitalyzer_ruleset},
        installation::Installation,
        ruleset::Ruleset,
    },
    tools::git::GitRepository,
    solar_error::SolarError,
    traits::Installable,
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct CommitalyzerInstaller {
    /// Rulesets to install.
    #[arg(short, long, num_args = 0..)]
    rulesets: Option<Vec<Ruleset>>,
}

impl Installable for CommitalyzerInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // if no ruleset is supplied, default to conventional-commits.
        let rulesets_to_install = match &self.rulesets {
            Some(rulesets) => rulesets,
            None => &vec![Ruleset::default()],
        };

        // Download binary if it doesn't already exist.
        if !git_repository.installation().hook_bin() {
            download_commitalyzer_binary(
                &path.join(git_repository.installation().hooks_path().path()),
            )?;
        }

        // Ensure commit rules directory exists.
        let commit_rules_path = path.join(COMMIT_RULES_NAME);
        fs::create_dir_all(&commit_rules_path)?;

        // Download rulesets that do not exist.
        for ruleset in rulesets_to_install {
            if !git_repository
                .installation()
                .rulesets()
                .contains(&ruleset.get().to_string())
            {
                download_commitalyzer_ruleset(&commit_rules_path, ruleset)?;
            }
        }

        Ok(())
    }
}
