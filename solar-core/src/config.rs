mod cargo_bin_basic;
mod cargo_lib_basic;
mod cargo_proc_basic;
mod universal_default;

pub use cargo_bin_basic::cargo_bin_basic;
pub use cargo_lib_basic::cargo_lib_basic;
pub use cargo_proc_basic::cargo_proc_basic;
use derive_setters::Setters;
pub use universal_default::universal_default;

use std::{fs, io::Write, path::PathBuf};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    SolarError, ToolTrait,
    tool::{CargoDeny, Commitalyzer, Licenses, PreCommit, SemverRelease, Vhooks, Workflows},
};

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum ProjConfig {
    CargoBinBasic,
    CargoLibBasic,
    CargoProcBasic,
    UniversalDefault,
}

impl ProjConfig {
    pub fn get(&self) -> Config {
        match self {
            Self::CargoBinBasic => cargo_bin_basic(),
            Self::CargoLibBasic => cargo_lib_basic(),
            Self::CargoProcBasic => cargo_proc_basic(),
            Self::UniversalDefault => universal_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Getters, Setters)]
pub struct Config {
    #[setters(rename = "set_vhooks")]
    vhooks: Option<Vhooks>,

    #[setters(rename = "set_semver_release")]
    semver_release: Option<SemverRelease>,

    #[setters(rename = "set_pre_commit")]
    pre_commit: Option<PreCommit>,

    #[setters(rename = "set_licenses")]
    licenses: Option<Licenses>,

    #[setters(rename = "set_github_workflows")]
    github_workflows: Option<Workflows>,

    #[setters(rename = "set_commitalyzer")]
    commitalyzer: Option<Commitalyzer>,

    #[setters(rename = "set_cargo_deny")]
    cargo_deny: Option<CargoDeny>,
}

impl ToolTrait for Config {
    fn set_dest(&mut self, dest: PathBuf) {
        self.on_all(|tool| tool.set_dest(dest.clone()));
    }

    fn act(&mut self, action: &crate::Action, dest: Option<PathBuf>) -> Result<(), SolarError> {
        self.try_all(|tool| tool.act(action, dest.clone()))?;
        Ok(())
    }

    fn install(&mut self) -> Result<(), SolarError> {
        self.try_all(|tool| tool.install())?;
        Ok(())
    }

    fn upgrade(&mut self) -> Result<(), SolarError> {
        self.try_all(|tool| tool.upgrade())?;
        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        self.try_all(|tool| tool.uninstall())?;
        Ok(())
    }
}

impl Config {
    pub fn new(
        vhooks: Option<Vhooks>,
        semver_release: Option<SemverRelease>,
        pre_commit: Option<PreCommit>,
        licenses: Option<Licenses>,
        github_workflows: Option<Workflows>,
        commitalyzer: Option<Commitalyzer>,
        cargo_deny: Option<CargoDeny>,
    ) -> Self {
        Self {
            vhooks,
            semver_release,
            pre_commit,
            licenses,
            github_workflows,
            commitalyzer,
            cargo_deny,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(None, None, None, None, None, None, None)
    }

    pub fn on_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut dyn ToolTrait),
    {
        if let Some(tool) = &mut self.vhooks {
            f(tool);
        }
        if let Some(tool) = &mut self.semver_release {
            f(tool);
        }
        if let Some(tool) = &mut self.pre_commit {
            f(tool);
        }
        if let Some(tool) = &mut self.licenses {
            f(tool);
        }
        if let Some(tool) = &mut self.github_workflows {
            f(tool);
        }
        if let Some(tool) = &mut self.commitalyzer {
            f(tool);
        }
        if let Some(tool) = &mut self.cargo_deny {
            f(tool);
        }
    }

    pub fn try_all<F>(&mut self, mut f: F) -> Result<(), SolarError>
    where
        F: FnMut(&mut dyn ToolTrait) -> Result<(), SolarError>,
    {
        if let Some(tool) = &mut self.vhooks {
            f(tool)?;
        }
        if let Some(tool) = &mut self.semver_release {
            f(tool)?;
        }
        if let Some(tool) = &mut self.pre_commit {
            f(tool)?;
        }
        if let Some(tool) = &mut self.licenses {
            f(tool)?;
        }
        if let Some(tool) = &mut self.github_workflows {
            f(tool)?;
        }
        if let Some(tool) = &mut self.commitalyzer {
            f(tool)?;
        }
        if let Some(tool) = &mut self.cargo_deny {
            f(tool)?;
        }
        Ok(())
    }

    /// Creates a new Config from a file at the supplied path, provided the file contains
    /// valid syntax for JSON and the config.
    pub fn load_from_file(file_path: PathBuf) -> Result<Self, SolarError> {
        let config_file = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }

    pub fn save_to_file(&self, file_path: PathBuf) -> Result<(), SolarError> {
        if !fs::exists(&file_path)? {
            fs::File::create(&file_path)?;
        }
        let mut file = fs::File::options().write(true).open(file_path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.vhooks.is_none()
            && self.semver_release.is_none()
            && self.pre_commit.is_none()
            && self.licenses.is_none()
            && self.github_workflows.is_none()
            && self.commitalyzer.is_none()
            && self.cargo_deny.is_none()
    }
}
