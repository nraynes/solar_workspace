mod cold;
mod cold_binaries;
mod cold_config;
mod hot;
mod hot_plugins;

use std::{collections::HashSet, path::Path};

use rust_terminal::Terminal;

use crate::{component_tests::semver_release::Snapshot, resources::CARGO_COMMAND};

pub fn test_upgrade_was_successful(path: &Path, plugins: Option<Vec<&str>>) {
    let mut command_args = vec!["solar", "upgrade", "semver-release"];
    if let Some(plugin_names) = &plugins {
        command_args.push("--plugins");
        command_args.extend(plugin_names);
    }

    // Run command.
    Terminal::command()
        .current_dir(path)
        .piped()
        .run(CARGO_COMMAND, command_args)
        .unwrap();

    let snapshot_after = Snapshot::from(path);

    // TODO: Test upgrade worked.
}
