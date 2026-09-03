use rust_terminal::Terminal;

use crate::solar_error::SolarError;

/// Install a global crate if it does not already exist.
pub fn is_crate_installed(crt: &str) -> Result<bool, SolarError> {
    // Get list of installed crates.
    let output = Terminal::command().run("cargo", ["install", "--list"])?;

    // Check if crate is in list.
    Ok(String::from_utf8(output.stdout)?.contains(crt))
}
