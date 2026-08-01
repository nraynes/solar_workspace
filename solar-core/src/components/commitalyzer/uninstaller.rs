use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::{
        commitalyzer::{
            COMMIT_MSG_NAME, COMMIT_RULES_NAME, installation::Installation, ruleset::Ruleset,
        },
        git::GitRepository,
    },
    solar_error::SolarError,
    traits::Uninstallable,
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct CommitalyzerUninstaller {
    /// Rulesets to uninstall.
    #[arg(short, long, num_args = 0..)]
    rulesets: Option<Vec<Ruleset>>,
}

impl Uninstallable for CommitalyzerUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // Remove binary.
        if *git_repository.installation().hook_bin() {
            fs::remove_file(
                git_repository
                    .installation()
                    .hooks_path()
                    .path()
                    .join(COMMIT_MSG_NAME),
            )?;
        }

        // Remove commit rulesets.
        let commit_rules_path = path.join(COMMIT_RULES_NAME);
        if fs::exists(&commit_rules_path)? {
            if let Some(rulesets) = &self.rulesets {
                for ruleset in rulesets {
                    if git_repository
                        .installation()
                        .rulesets()
                        .contains(&ruleset.get().to_string())
                    {
                        fs::remove_file(&commit_rules_path.join(ruleset.file_name()))?;
                    }
                }
            } else {
                fs::remove_dir_all(commit_rules_path)?;
            }
        }

        Ok(())
    }
}
