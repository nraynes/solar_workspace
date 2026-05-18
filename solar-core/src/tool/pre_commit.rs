use crate::{SolarError, ToolTrait};
use clap::Parser;
use rust_terminal::Terminal;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

static PRECOMMIT_SCRIPT_CONTENT: &str = "#!/bin/bash

# Pull changes first.
git pull

# Function to check if an element is present in an array.
has_element() {
    local e
    for e in \"${@:2}\"; do [[ \"$e\" == \"$1\" ]] && return 0; done
    return 1
}

# Get the current diff from git
diff_pre=($(git diff --name-only))

# Format files
cargo fmt

# Generate documentation
cargo doc

# Run tests
cargo test

# Check licenses
cargo deny check licenses

# Get the diff from git after formatting
diff_post=($(git diff --name-only))

# Check if any files were changed and add them to the current commit
for val in \"${diff_post[@]}\"; do
    if ! has_element \"$val\" \"${diff_pre[@]}\"; then
        git add \"$val\"
    fi
done
";

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct PreCommit {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,
}

impl PreCommit {
    fn precommit_path(&self) -> Result<PathBuf, SolarError> {
        let output = Terminal::command().run("git", vec!["config", "core.hooksPath"])?;
        let git_hooks_path = PathBuf::from(String::from_utf8(output.stdout)?);
        Ok(self
            .working_dir
            .join(git_hooks_path.join(PathBuf::from("pre-commit"))))
    }
}

impl ToolTrait for PreCommit {
    fn install(&self) -> Result<(), SolarError> {
        let mut precommit_file = File::create(self.precommit_path()?)?;
        precommit_file.write_all(PRECOMMIT_SCRIPT_CONTENT.as_bytes())?;
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        fs::remove_file(self.precommit_path()?)?;
        Ok(())
    }
}
