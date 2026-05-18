use std::string::FromUtf8Error;

use rust_alert::alert;

/// A custom error type used to convert error types from various crates.
#[alert(errors = [std::io::Error, rust_terminal::TerminalError, reqwest::Error, url::ParseError, FromUtf8Error])]
pub struct SolarError {}
