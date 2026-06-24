use crate::{Config, SOLARCONFIGNAME, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use toml::{Value, map::Map};

static TOML_NAME: &str = "deny.toml";

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct CargoDeny {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// Default licenses to allow in your dependencies in your project.
    #[arg(short, long, num_args = 0..)]
    allow_licenses: Option<Vec<String>>,
}

impl CargoDeny {
    pub fn new(destination: PathBuf, allow_licenses: Option<Vec<String>>) -> Self {
        Self {
            destination,
            allow_licenses,
        }
    }

    fn ensure_tool_installed(&self) -> Result<(), SolarError> {
        let output = Terminal::command()
            .current_dir(&self.destination)
            .run("cargo", ["install", "--list"])?;
        if !String::from_utf8(output.stdout)?.contains("cargo-deny") {
            Terminal::command()
                .current_dir(&self.destination)
                .piped()
                .run("cargo", ["install", "cargo-deny"])?;
        }
        Ok(())
    }

    fn generate_toml(&self) -> Result<String, SolarError> {
        let allowed_licenses = self
            .allow_licenses
            .as_ref()
            .ok_or("No allowed licenses were specified.")?;
        let mut toml_config: Map<String, Value> = Map::new();
        let mut allow_list = HashMap::new();
        allow_list.insert("allow", allowed_licenses.clone());
        let licenses_section = Value::from(allow_list);
        toml_config.insert("licenses".to_string(), licenses_section);
        Ok(toml::to_string_pretty(&toml_config)?)
    }
}

impl ToolTrait for CargoDeny {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        // Check for current toml configuration.
        if fs::exists(self.destination.join(TOML_NAME))? {
            return Err(SolarError::from(
                "Current installation found. Use cargo-deny to make changes to the current installation. Only use cargo-solar to uninstall completely.",
            ));
        }

        // Get or make configuration file.
        let config = Config::new_empty(&self.destination.join(SOLARCONFIGNAME));

        self.ensure_tool_installed()?;

        // Generate config file.
        let toml_contents = &self.generate_toml()?.into_bytes();

        // Create configuration file.
        let mut deny_config = File::create(self.destination.join(TOML_NAME))?;
        deny_config.write_all(toml_contents)?;

        config.set_cargo_deny(Some(self.clone())).save()?;

        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        let config = Config::load_from(&self.destination)?;
        config
            .cargo_deny()
            .as_ref()
            .ok_or("No cargo_deny configuration inside config file.")?;

        // Remove configuration file.
        fs::remove_file(self.destination.join("deny.toml"))?;

        // Update configuration, remove if empty.
        let config = config.set_cargo_deny(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
