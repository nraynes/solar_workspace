use rust_terminal::Terminal;

use crate::{components::cargo::is_crate_installed::is_crate_installed, solar_error::SolarError};

/// Install a global crate if it does not already exist.
pub fn try_cargo_install(crt: &str) -> Result<(), SolarError> {
    if !is_crate_installed(crt)? {
        Terminal::command().piped().run("cargo", ["install", crt])?;
    }
    Ok(())
}
