use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::pre_commit::{PRE_COMMIT, Script, installation::Installation},
    solar_error::SolarError,
    tools::git::GitRepository,
    traits::Installable,
};

pub static PRECOMMIT_ALREADY_EXISTS_ERR_MSG: &str = "There is already a pre-commit hook present. Please use the -f or --force-overwrite option to force overwriting the old hook.";

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct PreCommitInstaller {
    // The script to use as the pre-commit hook.
    #[arg(short, long)]
    script: Script,

    // Whether to force overwriting the old script if there is one.
    #[arg(short, long)]
    force_overwrite: bool,
}

impl Installable for PreCommitInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let git_repository = GitRepository::<Installation>::from_path(path)?;

        // If a hook is already present and force_overwrite is not set, return error.
        if *git_repository.installation().script_exists() && !self.force_overwrite {
            return Err(SolarError::from(PRECOMMIT_ALREADY_EXISTS_ERR_MSG));
        }

        // Create the new hook file.
        let mut precommit_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(
                git_repository
                    .installation()
                    .hooks_path()
                    .path()
                    .join(PRE_COMMIT),
            )?;
        precommit_file.write_all(self.script.content().as_bytes())?;

        Ok(())
    }
}
