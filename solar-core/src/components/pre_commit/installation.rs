use std::{fs, path::Path};

use derive_getters::Getters;

use crate::{
    components::pre_commit::PRE_COMMIT,
    tools::git::HooksPath,
    solar_error::SolarError,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    hooks_path: HooksPath,
    script_exists: bool,
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        let hooks_path = HooksPath::try_from(path)?;
        let script_exists = fs::exists(hooks_path.path().join(PRE_COMMIT))?;
        Ok(Self {
            hooks_path,
            script_exists,
        })
    }
}
