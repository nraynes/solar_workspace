use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn default_allow_licenses() -> Vec<String> {
    vec![
        "MIT".to_string(),
        "Apache-2.0".to_string(),
        "Unicode-3.0".to_string(),
    ]
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct CargoDeny {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(default = "Global::default_destination")]
    destination: PathBuf,

    /// Default licenses to allow in your dependencies in your project.
    #[arg(short, long, default_values = ["MIT", "Apache-2.0", "Unicode-3.0"])]
    #[serde(default = "default_allow_licenses")]
    allow_licenses: Vec<String>,
}

impl CargoDeny {
    pub fn new(destination: PathBuf, allow_licenses: Vec<String>) -> Self {
        Self {
            destination,
            allow_licenses,
        }
    }

    fn ensure_tool_installed(&self) -> Result<(), SolarError> {
        let output = Terminal::command()
            .current_dir(self.destination.clone())
            .run("cargo", ["install", "--list"])?;
        if !String::from_utf8(output.stdout)?.contains("cargo-deny") {
            Terminal::command()
                .current_dir(self.destination.clone())
                .piped()
                .run("cargo", ["install", "cargo-deny"])?;
        }
        Ok(())
    }

    fn deny_config_content(&self) -> String {
        let part_one = String::from("[licenses]\nallow = [\n");
        let mut part_two = String::new();
        let part_three = String::from("]\n");
        for license in &self.allow_licenses {
            part_two.push_str(&format!("\t\"{}\",\n", license));
        }
        format!("{}{}{}", part_one, part_two, part_three)
    }
}

impl ToolTrait for CargoDeny {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
        self.ensure_tool_installed()?;

        // Create configuration file.
        let mut deny_config = File::create(self.destination.join(PathBuf::from("deny.toml")))?;
        deny_config.write_all(&self.deny_config_content().into_bytes())?;

        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Remove configuration file.
        fs::remove_file(self.destination.join(PathBuf::from("deny.toml")))?;

        Ok(())
    }
}
