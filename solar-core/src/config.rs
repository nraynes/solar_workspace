mod cargo_bin_basic;
mod cargo_lib_basic;
mod cargo_proc_basic;
mod universal_default;

pub use cargo_bin_basic::cargo_bin_basic;
pub use cargo_lib_basic::cargo_lib_basic;
pub use cargo_proc_basic::cargo_proc_basic;
use derive_setters::Setters;
pub use universal_default::universal_default;

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    SOLARCONFIGNAME, SolarError, ToolTrait,
    tool::{CargoDeny, Commitalyzer, GithubWorkflows, Licenses, PreCommit, SemverRelease, Vhooks},
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
    #[serde(skip)]
    #[setters(rename = "set_path")]
    path: PathBuf,

    #[setters(rename = "set_vhooks")]
    vhooks: Option<Vhooks>,

    #[setters(rename = "set_semver_release")]
    semver_release: Option<SemverRelease>,

    #[setters(rename = "set_pre_commit")]
    pre_commit: Option<PreCommit>,

    #[setters(rename = "set_licenses")]
    licenses: Option<Licenses>,

    #[setters(rename = "set_github_workflows")]
    github_workflows: Option<GithubWorkflows>,

    #[setters(rename = "set_commitalyzer")]
    commitalyzer: Option<Commitalyzer>,

    #[setters(rename = "set_cargo_deny")]
    cargo_deny: Option<CargoDeny>,
}

impl ToolTrait for Config {
    fn set_dest(&mut self, dest: &Path) {
        self.on_all(|tool| tool.set_dest(dest));
    }

    fn act(&mut self, action: &crate::Action, dest: Option<&Path>) -> Result<(), SolarError> {
        self.try_all(|tool| tool.act(action, dest))?;
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: PathBuf,
        vhooks: Option<Vhooks>,
        semver_release: Option<SemverRelease>,
        pre_commit: Option<PreCommit>,
        licenses: Option<Licenses>,
        github_workflows: Option<GithubWorkflows>,
        commitalyzer: Option<Commitalyzer>,
        cargo_deny: Option<CargoDeny>,
    ) -> Self {
        Self {
            path,
            vhooks,
            semver_release,
            pre_commit,
            licenses,
            github_workflows,
            commitalyzer,
            cargo_deny,
        }
    }

    pub fn new_empty(dest: &Path) -> Self {
        Self::new(dest.to_path_buf(), None, None, None, None, None, None, None)
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
    pub fn load_from_file(file_path: &Path) -> Result<Self, SolarError> {
        let config_file = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config.set_path(file_path.to_path_buf()))
    }

    pub fn load_from(file_path: &Path) -> Result<Self, SolarError> {
        Self::load_from_file(&file_path.join(SOLARCONFIGNAME))
    }

    pub fn load_or_default(file_path: &Path) -> Self {
        Self::load_from(file_path).unwrap_or(Self::new_empty(&file_path.join(SOLARCONFIGNAME)))
    }

    pub fn save_to_file(&self, file_path: &Path) -> Result<(), SolarError> {
        if !fs::exists(file_path)? {
            fs::File::create(file_path)?;
        }
        let mut file = fs::File::options()
            .write(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn save_to(&self, file_path: &Path) -> Result<(), SolarError> {
        self.save_to_file(&file_path.join(SOLARCONFIGNAME))
    }

    pub fn save(&self) -> Result<(), SolarError> {
        self.save_to_file(&self.path)
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
