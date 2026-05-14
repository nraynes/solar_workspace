use rust_alert::alert;
use rust_terminal::TerminalError;

/// A custom error type used to convert error types from various crates.
#[alert(errors = [std::io::Error, TerminalError])]
pub struct SolarError {}
