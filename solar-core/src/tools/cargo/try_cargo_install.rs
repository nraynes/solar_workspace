use rust_terminal::Terminal;

use crate::solar_error::SolarError;

/// Install a global crate if it does not already exist.
pub fn try_cargo_install(crt: &str) -> Result<(), SolarError> {
    Terminal::command().piped().run("cargo", ["install", crt])?;
    Ok(())
}
