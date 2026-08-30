use std::{collections::HashSet, path::Path};

use rust_terminal::Terminal;

use crate::{component_tests::semver_release::Snapshot, resources::CARGO_COMMAND};

mod cold;
mod cold_binaries;
mod cold_config;
mod hot;
mod hot_plugins;

pub fn test_uninstall_was_successful(path: &Path, plugins: Option<Vec<&str>>) {
    let mut command_args = vec!["solar", "uninstall", "semver-release"];
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

    if let Some(plugin_names) = plugins {
        let config = snapshot_after
            .plugin_configurations()
            .unwrap()
            .keys()
            .collect::<HashSet<&String>>();

        for name in plugin_names {
            assert!(config.get(&name.to_string()).is_none());
        }
    } else {
        assert!(snapshot_after.release_dir().is_none());
        assert!(snapshot_after.semver_config().is_none());
    }
}
