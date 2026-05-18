use crate::{SolarError, ToolTrait};
use clap::Parser;
use rust_terminal::Terminal;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct CargoDeny {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// Default licenses to allow in your dependencies in your project.
    #[arg(short, long, default_values = ["MIT", "Apache-2.0", "Unicode-3.0"])]
    allow_licenses: Vec<String>,
}

impl CargoDeny {
    fn ensure_tool_installed() -> Result<(), SolarError> {
        let output = Terminal::command().run("cargo", ["install", "--list"])?;
        if !String::from_utf8(output.stdout)?.contains("cargo-deny") {
            Terminal::command()
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
    fn install(&self) -> Result<(), SolarError> {
        Self::ensure_tool_installed()?;

        // Create configuration file.
        let mut deny_config = File::create(self.working_dir.join(PathBuf::from("deny.toml")))?;
        deny_config.write_all(&self.deny_config_content().into_bytes())?;

        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Remove configuration file.
        fs::remove_file(self.working_dir.join(PathBuf::from("deny.toml")))?;

        Ok(())
    }
}
