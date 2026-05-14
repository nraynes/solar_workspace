use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use rust_terminal::Terminal;
use std::path::PathBuf;

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Vhooks {
    /// Git hooks directory name.
    #[arg(short, long, default_value = ".hooks")]
    name: String,

    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// Whether to remove all hooks when removing vhooks, or just put thim in unversioned git hooks directory.
    #[arg(short, long, default_value = "false")]
    remove_all: bool,
}

impl Vhooks {
    fn pathbuf_to_str(&self, path: PathBuf) -> Result<String, SolarError> {
        Ok(String::from(
            path.to_str()
                .ok_or("Could not convert path buffer to string")?,
        ))
    }
}

impl ToolTrait for Vhooks {
    fn install(&self) -> Result<(), SolarError> {
        // Path to the versioned hooks directory.
        let hooks_path = self.pathbuf_to_str(self.working_dir.join(&self.name))?;

        // Create the new hooks directory.
        Terminal::command()
            .piped()
            .run("mkdir", ["-p", &hooks_path])?;

        // Ensure working directory is a git repository.
        Global::git_init(&self.working_dir)?;

        // Set the new hooks directory as the git hooks directory.
        Terminal::command()
            .piped()
            .run("git", vec!["config", "core.hooksPath", &hooks_path])?;
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Paths to the versioned hooks directory and default directory.
        let hooks_path = self.pathbuf_to_str(self.working_dir.join(&self.name).join("*"))?;
        let default_path = self.pathbuf_to_str(Global::default_git_hook_dir())?;

        // Must be a git repository in order to set default hook directory.
        Global::git_init(&self.working_dir)?;

        // Git hooks folder must exist.
        Terminal::command()
            .piped()
            .run("mkdir", ["-p", &default_path])?;

        // If not removing hooks, move them to the default hooks directory.
        if !self.remove_all {
            Terminal::command()
                .piped()
                .run("mv", [&hooks_path, &default_path])?;
        }

        // Set the new hooks directory as the default git hooks directory.
        Terminal::command()
            .piped()
            .run("git", vec!["config", "core.hooksPath", &default_path])?;

        // Remove the versioned hooks folder.
        Terminal::command()
            .piped()
            .run("rm", ["-rf", &hooks_path])?;
        Ok(())
    }
}
