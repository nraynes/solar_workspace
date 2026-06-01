use std::path::PathBuf;

use crate::{
    Config,
    tool::{Commitalyzer, Licenses, SemverRelease, Vhooks},
};

pub fn universal_default() -> Config {
    Config::new(
        Some(Vhooks::new(PathBuf::from("."), ".hooks".to_string(), false)),
        Some(SemverRelease::new(PathBuf::from("."), vec![])),
        None,
        Some(Licenses::new(
            PathBuf::from("."),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
        )),
        None,
        Some(Commitalyzer::new(PathBuf::from("."))),
        None,
    )
}
