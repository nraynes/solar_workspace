use std::path::Path;

use rust_terminal::Terminal;

use crate::{component_tests::vhooks::Snapshot, resources::CARGO_COMMAND};

mod cold;
mod hot;
mod hot_remove_hooks;
mod no_git;

pub fn ensure_install_succeeds_first(path: &Path) {
    let snapshot_before_install = Snapshot::from(path);
    let hooks_dir_before_install = snapshot_before_install.hooks_dir().as_ref().unwrap();

    // Install Vhooks first.
    Terminal::command()
        .current_dir(path)
        .piped()
        .run(
            CARGO_COMMAND,
            ["solar", "install", "vhooks", "--hooks-path", ".hooks"],
        )
        .unwrap();

    // Get file system snapshot before command runs.
    let snapshot_after_install = Snapshot::from(path);

    // Ensure that install at least succeeded before running uninstall.
    if snapshot_after_install.hooks_dir().as_ref().is_none_or(|x| {
        x.path() != &path.join(".hooks") || !x.deep_eq().dir_weak(hooks_dir_before_install).unwrap()
    }) {
        panic!(
            "Install Vhooks did not succeed when running uninstall test. Cannot test uninstall Vhooks."
        );
    }
}
