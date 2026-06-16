use std::path::PathBuf;

use crate::{
    Config, GithubWorkflows,
    tool::{
        CargoDeny, Commitalyzer, Licenses, Plugin, PreCommit, SemverRelease, Vhooks, Workflow,
        pre_commit::Script,
    },
};

pub fn cargo_lib_basic() -> Config {
    Config::new(
        PathBuf::new(),
        Some(Vhooks::new(PathBuf::from("."), ".hooks".to_string(), false)),
        Some(SemverRelease::new(PathBuf::from("."), vec![Plugin::Cargo])),
        Some(PreCommit::new(PathBuf::from("."), Some(Script::CargoBasic))),
        Some(Licenses::new(
            PathBuf::from("."),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
            Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
        )),
        Some(GithubWorkflows::new(
            PathBuf::from("."),
            Some(vec![
                Workflow::ReleaseCargoLibGeneral,
                Workflow::TestCargoGeneral,
            ]),
        )),
        Some(Commitalyzer::new(
            PathBuf::from("."),
            Some("conventional-commits".to_string()),
        )),
        Some(CargoDeny::new(
            PathBuf::from("."),
            Some(vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "Unicode-3.0".to_string(),
            ]),
        )),
    )
}
