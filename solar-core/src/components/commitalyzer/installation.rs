use std::{collections::HashSet, fs, path::Path, str::FromStr};

use derive_getters::Getters;

use crate::{
    components::{
        commitalyzer::{COMMIT_RULES_NAME, ruleset::Ruleset},
        git::HooksPath,
    },
    solar_error::SolarError,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    hooks_path: HooksPath,
    hook_bin: bool,
    rulesets: HashSet<String>,
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        // Get hooks path and commitalyzer binary.
        let hooks_path = HooksPath::try_from(path)?;
        let hook_bin = fs::exists(path.join(hooks_path.path()))?;

        let mut rulesets = HashSet::new();
        let read_dir_result = fs::read_dir(path.join(COMMIT_RULES_NAME));

        if let Ok(read_dir) = read_dir_result {
            for dir_entry_result in read_dir {
                if let Ok(dir_entry) = dir_entry_result
                    && let Some(file_stem) = dir_entry.path().file_stem()
                    && let Some(file_name) = file_stem.to_str()
                    && let Ok(ruleset) = Ruleset::from_str(file_name)
                {
                    rulesets.insert(ruleset.get().to_string());
                }
            }
        }

        Ok(Self {
            hooks_path,
            hook_bin,
            rulesets,
        })
    }
}
