pub mod is_crate_installed;
pub mod try_cargo_install;

mod crate_builder;

pub use crate_builder::{CrateBuilder};

pub static CARGO_TOML: &str = "Cargo.toml";
