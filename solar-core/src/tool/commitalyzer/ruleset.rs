use std::fmt::Display;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Copy)]
pub enum Ruleset {
    ConventionalCommits,
}

impl Display for Ruleset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Self::ConventionalCommits => "conventional-commits",
            }
        )
    }
}
