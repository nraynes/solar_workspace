use std::path::Path;

use rust_terminal::Terminal;

use affirm_fs::DirStructure;
use solar_core::components::semver_release::RELEASE_DIR_NAME;

use crate::{
    component_tests::semver_release::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

fn test_install_was_successful(path: &Path, plugins: Option<Vec<&str>>) {
    let mut command_args = vec!["solar", "install", "semver-release"];
    let mut structure = DirStructure::new(path.join(RELEASE_DIR_NAME)).file("semver-release");
    if let Some(plugin_names) = plugins {
        command_args.push("--plugins");
        command_args.extend(&plugin_names);

        for name in plugin_names {
            structure = structure.file(name);
        }
    }
    let mock_release_dir = structure.build();

    // Run command.
    Terminal::command()
        .current_dir(path)
        .piped()
        .run(CARGO_COMMAND, command_args)
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(path);
    let release_dir_after = snapshot_after.release_dir().as_ref().unwrap();

    assert!(release_dir_after.eq().dir(&mock_release_dir))
}

#[test]
pub fn semver_release() {
    let mut temp = setup_env();

    test_install_was_successful(temp.env().path(), None);
    test_install_was_successful(temp.env().path(), None);
}
