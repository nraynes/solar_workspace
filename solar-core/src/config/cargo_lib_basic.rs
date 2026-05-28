use std::path::PathBuf;

use crate::{
    Config,
    tool::{
        CargoDeny, Commitalyzer, Licenses, Plugin, PreCommit, ReleaseWfType, SemverRelease,
        TestWfType, Vhooks, Workflows,
    },
};

pub fn cargo_lib_basic(destination: PathBuf) -> Config {
    Config::new(
        Some(Vhooks::new(
            destination.clone(),
            ".hooks".to_string(),
            false,
        )),
        Some(SemverRelease::new(destination.clone(), vec![Plugin::CARGO])),
        Some(PreCommit::new(destination.clone())),
        Some(Licenses::new(
            destination.clone(),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
        )),
        Some(Workflows::new(
            destination.clone(),
            Some(ReleaseWfType::BIN),
            Some(TestWfType::GENERAL),
        )),
        Some(Commitalyzer::new(destination.clone())),
        Some(CargoDeny::new(
            destination.clone(),
            vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "Unicode-3.0".to_string(),
            ],
        )),
    )
}
