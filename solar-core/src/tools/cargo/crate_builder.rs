mod dependency;

use dependency::Dependency;
use derive_getters::Getters;
use rust_terminal::Terminal;
use toml::{Table, Value, map::Map};

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use derive_new::new;

use crate::{solar_error::SolarError, tools::cargo::CARGO_TOML};

#[derive(Getters, new)]
pub struct CrateBuilder {
    path: PathBuf,
    name: String,
    version: (u32, u32, u32),
    authors: Vec<String>,
    description: String,
    license: String,
    repository: String,
    keywords: Vec<String>,
    categories: Vec<String>,

    #[new(into)]
    dependencies: Vec<Dependency>,
}

impl CrateBuilder {
    fn cargo<const N: usize>(&self, args: [&str; N]) -> Result<(), SolarError> {
        Terminal::command()
            .piped()
            .current_dir(&self.path)
            .run("cargo", args)?;
        Ok(())
    }

    fn crate_path(&self) -> PathBuf {
        self.path.join(&self.name)
    }

    fn in_workspace(&self) -> Result<bool, SolarError> {
        if fs::exists(self.path.join(CARGO_TOML))? {
            let cargo_toml = Self::get_cargo_toml(&self.path)?;
            if cargo_toml.contains_key("workspace") {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn add_dependencies<const N: usize>(
        &self,
        additional_dependencies: [Dependency; N],
    ) -> Result<(), SolarError> {
        for dependency in &self.dependencies {
            dependency.add_to(&self.path)?;
        }

        for dependency in additional_dependencies {
            dependency.add_to(&self.path)?;
        }

        Ok(())
    }

    fn package_extension(
        &self,
        include_files_with: Option<Vec<Value>>,
    ) -> Result<Map<String, Value>, SolarError> {
        let mut package_extension_table: Map<String, Value> = Map::new();
        let mut include_files = vec![
            Value::String("src/".into()),
            Value::String("examples/".into()),
            Value::String("Cargo.toml".into()),
            if self.in_workspace()? {
                Value::String("../README.md".into())
            } else {
                Value::String("README.md".into())
            },
        ];
        if let Some(include_files_extension) = include_files_with {
            include_files.extend(include_files_extension);
        }
        package_extension_table.insert("include".into(), Value::Array(include_files));
        Ok(package_extension_table)
    }

    pub fn get_cargo_toml(path: &Path) -> Result<Map<String, Value>, SolarError> {
        let cargo_toml_string = fs::read_to_string(path.join(CARGO_TOML))?;
        Ok(cargo_toml_string.parse::<Table>()?)
    }

    pub fn save_cargo_toml(
        path: &Path,
        new_cargo_toml: &Map<String, Value>,
    ) -> Result<(), SolarError> {
        let mut file_handle = File::options()
            .write(true)
            .truncate(true)
            .open(path.join(CARGO_TOML))?;
        file_handle.write_all(toml::to_string_pretty(new_cargo_toml)?.as_bytes())?;
        Ok(())
    }

    pub fn include_files_ref(
        cargo_toml: &mut Map<String, Value>,
    ) -> Result<&mut Vec<Value>, SolarError> {
        Ok(cargo_toml
            .get_mut("package")
            .ok_or("The Cargo.toml file did not have the expected layout.")?
            .as_table_mut()
            .ok_or("'package' section does not have the expected layout.")?
            .get_mut("include")
            .ok_or("Something went wrong. No include files list found in Cargo.toml?")?
            .as_array_mut()
            .ok_or("Include files list in Cargo.toml is not an array.")?)
    }

    fn build_cargo_toml(
        &self,
        extend_with: Option<Map<String, Value>>,
        extend_package_table_with: Option<Map<String, Value>>,
    ) -> Result<(), SolarError> {
        let crate_path = &self.crate_path();
        let mut cargo_toml: Map<String, Value> = Self::get_cargo_toml(crate_path)?;
        let package_table = cargo_toml
            .get_mut("package")
            .ok_or("The Cargo.toml file did not have the expected layout.")?
            .as_table_mut()
            .ok_or("'package' section does not have the expected layout.")?;

        package_table.insert(
            "version".into(),
            Value::String(format!(
                "{}.{}.{}",
                self.version.0, self.version.1, self.version.2
            )),
        );
        package_table.insert(
            "authors".into(),
            Value::Array(
                self.authors
                    .to_owned()
                    .into_iter()
                    .map(|s| Value::String(s))
                    .collect(),
            ),
        );
        package_table.insert(
            "description".into(),
            Value::String(self.description.to_owned()),
        );
        package_table.insert("license".into(), Value::String(self.license.to_owned()));
        package_table.insert(
            "repository".into(),
            Value::String(self.repository.to_owned()),
        );
        package_table.insert(
            "keywords".into(),
            Value::Array(
                self.keywords
                    .to_owned()
                    .into_iter()
                    .map(|s| Value::String(s))
                    .collect(),
            ),
        );
        package_table.insert(
            "categories".into(),
            Value::Array(
                self.categories
                    .to_owned()
                    .into_iter()
                    .map(|s| Value::String(s))
                    .collect(),
            ),
        );

        if let Some(package_table_extension) = extend_package_table_with {
            package_table.extend(package_table_extension);
        }

        if let Some(extension) = extend_with {
            cargo_toml.extend(extension);
        }

        Self::save_cargo_toml(crate_path, &cargo_toml)?;

        Ok(())
    }

    pub fn bin(&self) -> Result<(), SolarError> {
        // Build new bin crate.
        self.cargo(["new", &self.name])?;

        // Build Cargo.toml.
        self.build_cargo_toml(
            None,
            Some(self.package_extension(Some(vec![Value::String("build.rs".into())]))?),
        )?;

        // Add dependencies.
        self.add_dependencies([])?;
        Ok(())
    }

    pub fn lib(&self) -> Result<(), SolarError> {
        // Build new lib crate.
        self.cargo(["new", "--lib", &self.name])?;

        // Build Cargo.toml.
        self.build_cargo_toml(None, Some(self.package_extension(None)?))?;

        // Add dependencies.
        self.add_dependencies([])?;
        Ok(())
    }

    pub fn proc(&self) -> Result<(), SolarError> {
        // Build new lib crate.
        self.lib()?;

        // Build the lib table specifying crate as a proc macro.
        let mut cargo_toml_extension: Map<String, Value> = Map::new();
        let mut lib_table: Map<String, Value> = Map::new();
        lib_table.insert("proc-macro".into(), Value::Boolean(true));
        cargo_toml_extension.insert("lib".into(), Value::Table(lib_table));

        // Build Cargo.toml.
        self.build_cargo_toml(
            Some(cargo_toml_extension),
            Some(self.package_extension(None)?),
        )?;

        // Add dependencies + dependencies for proc macro.
        self.add_dependencies([
            Dependency::from(("proc-macro2", [])),
            Dependency::from(("quote", [])),
            Dependency::from(("syn", ["full"])),
        ])?;
        Ok(())
    }

    pub fn workspace(&self) -> Result<(), SolarError> {
        let workspace_path = self.path.join(&self.name);
        fs::create_dir_all(&workspace_path)?;

        // Build Cargo.toml for workspace.
        let mut cargo_toml: Map<String, Value> = Map::new();
        let mut workspace_table: Map<String, Value> = Map::new();
        workspace_table.insert("resolver".into(), Value::String("3".into()));
        cargo_toml.insert("workspace".into(), Value::Table(workspace_table));

        // Write Cargo.toml.
        let mut file_handle = File::options()
            .create_new(true)
            .write(true)
            .open(workspace_path.join(CARGO_TOML))?;
        file_handle.write_all(toml::to_string_pretty(&cargo_toml)?.as_bytes())?;

        Ok(())
    }
}
