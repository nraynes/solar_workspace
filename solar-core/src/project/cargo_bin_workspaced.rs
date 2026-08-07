use std::path::Path;

use clap::Parser;
use toml::Value;

use crate::{solar_error::SolarError, tools::cargo::CrateBuilder, traits::ConfigureProject};

#[derive(Parser, Clone)]
pub struct CargoBinWorkspaced {
    /// The authors for this crate, if any.
    #[arg(short, long)]
    authors: Option<Vec<String>>,

    /// The description for this crate, if any.
    #[arg(short, long)]
    description: Option<String>,

    /// The repository for this crate, if any.
    #[arg(short, long)]
    repository: Option<String>,

    /// The keywords for this crate, if any.
    #[arg(short, long)]
    keywords: Option<Vec<String>>,

    /// The categories for this crate, if any.
    #[arg(short, long)]
    categories: Option<Vec<String>>,

    /// If there is already a pre-commit hook present, this option will allow it to be overwritten.
    #[arg(short)]
    force_overwrite_pre_commit: bool,
}

impl ConfigureProject for CargoBinWorkspaced {
    fn deinit(&self, _: &Path) -> Result<(), SolarError> {
        Ok(())
    }

    fn new(&self, path: &Path, name: &str) -> Result<(), SolarError> {
        // Initialize cargo bin package.
        let cratebuilder = CrateBuilder::new(
            path.join(name),
            name.into(),
            (0, 0, 0),
            self.authors.clone().unwrap_or(Vec::new()),
            self.description.clone().unwrap_or("".into()),
            "MIT OR Apache-2.0".into(),
            self.repository.clone().unwrap_or("".into()),
            self.keywords.clone().unwrap_or(Vec::new()),
            self.categories.clone().unwrap_or(Vec::new()),
            vec![],
        );
        cratebuilder.bin()?;

        self.init(cratebuilder.path())
    }

    fn init(&self, path: &Path) -> Result<(), SolarError> {
        // Update Cargo.toml to include new files in published crate.
        let mut cargo_toml = CrateBuilder::get_cargo_toml(path)?;
        let include_files_list = CrateBuilder::include_files_ref(&mut cargo_toml)?;
        include_files_list.extend(vec![
            Value::String("../LICENSES/".into()),
            Value::String("../LICENSE-MIT".into()),
            Value::String("../LICENSE-Apache-2.0".into()),
            Value::String("../CHANGELOG.md".into()),
        ]);
        CrateBuilder::save_cargo_toml(path, &cargo_toml)?;

        Ok(())
    }

    fn update(&self, _: &Path) -> Result<(), SolarError> {
        Ok(())
    }
}
