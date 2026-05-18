use crate::{SPDX, SolarError, ToolTrait};
use clap::Parser;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

static LICENSE_MAIN: &str = "LICENSE";
static LICENSES_DIR: &str = "LICENSES";

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Licenses {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// The licenses to include in your project.
    #[arg(short, long, default_values = ["MIT", "Apache-2.0"])]
    include_licenses: Vec<SPDX>,

    /// The text to include in the main license file.
    #[arg(short, long, default_value = "MIT OR Apache-2.0")]
    licensed_under: String,

    /// Just grab all of the licenses.
    #[arg(short, long)]
    all: bool,
}

impl ToolTrait for Licenses {
    fn install(&self) -> Result<(), SolarError> {
        let licenses_dir = self.working_dir.join(PathBuf::from(LICENSES_DIR));

        // Make a new licenses folder.
        fs::create_dir_all(&licenses_dir)?;

        // Write the license files.
        for license in self.include_licenses.iter() {
            let mut license_file =
                File::create(licenses_dir.join(PathBuf::from(format!("LICENSE-{}", license))))?;
            license_file.write_all(license.get().as_bytes())?;
        }

        // Add the main license file.
        let mut main_license_file =
            File::create(self.working_dir.join(PathBuf::from(LICENSE_MAIN)))?;
        main_license_file.write_all(self.licensed_under.as_bytes())?;

        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Delete main license file.
        fs::remove_file(self.working_dir.join(PathBuf::from(LICENSE_MAIN)))?;

        // Delete the licenses folder along with its contents.
        fs::remove_dir_all(self.working_dir.join(PathBuf::from(LICENSES_DIR)))?;
        Ok(())
    }
}
