use std::{
    fs::{self, File},
    path::PathBuf,
};

use rust_terminal::Terminal;

use clap::Parser;

use crate::{SolarError, Install};

#[derive(Parser, Clone)]
pub struct Init {
    /// The destination to initialize the project.
    #[arg(short, long, default_value = ".")]
    destination: PathBuf,

    /// Whether to force reinitialization of an already existing git repository.
    /// WARNING: Setting this to true will delete commit history for existing git repositories.
    #[arg(short, long, default_value = "false")]
    force_init: bool,

    /// Whether to force tool installation to an existing git repository.
    #[arg(short, long, default_value = "false")]
    install_existing: bool,
}

impl Init {
    pub fn run(&self) -> Result<(), SolarError> {
        // Initialize git repository if forcing initialization or repository is not initialized already
        let no_existing_git_repo =
            fs::exists(self.destination.join(PathBuf::from(".git"))).is_err();
        if self.force_init || no_existing_git_repo {
            Terminal::command().piped().run("git", vec!["init"])?;
        }

        // Create a README.md file
        File::create(&self.destination.join(PathBuf::from("README.md")))?;

        // Install all tools into the project
        if self.install_existing || no_existing_git_repo {
            let installer: Install = Install::parse_from(vec![
                "",
                self.destination
                    .to_str()
                    .ok_or("Failed to convert destination path to string.")?,
            ]);
            installer.run()?;
        }
        Ok(())
    }
}
