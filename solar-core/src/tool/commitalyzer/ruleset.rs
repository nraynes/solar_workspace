use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Serialize, Eq, Deserialize, PartialEq, Debug, Copy)]
pub enum Ruleset {
    ConventionalCommits,
}

impl Ruleset {
    pub fn get(&self) -> &str {
        match self {
            Self::ConventionalCommits => "conventional-commits",
        }
    }
}
