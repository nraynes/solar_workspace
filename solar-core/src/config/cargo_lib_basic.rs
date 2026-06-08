use std::path::PathBuf;

use crate::{
    Config,
    tool::{
        CargoDeny, Commitalyzer, Licenses, Plugin, PreCommit, ReleaseWfType, SemverRelease,
        TestWfType, Vhooks, Workflows,
    },
};

pub fn cargo_lib_basic() -> Config {
    Config::new(
        PathBuf::new(),
        Some(Vhooks::new(PathBuf::from("."), ".hooks".to_string(), false)),
        Some(SemverRelease::new(PathBuf::from("."), vec![Plugin::Cargo])),
        Some(PreCommit::new(PathBuf::from("."))),
        Some(Licenses::new(
            PathBuf::from("."),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
        )),
        Some(Workflows::new(
            PathBuf::from("."),
            Some(ReleaseWfType::Lib),
            Some(TestWfType::General),
        )),
        Some(Commitalyzer::new(PathBuf::from("."))),
        Some(CargoDeny::new(
            PathBuf::from("."),
            vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "Unicode-3.0".to_string(),
            ],
        )),
    )
}
