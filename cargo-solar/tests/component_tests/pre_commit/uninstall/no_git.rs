use affirm_fs::contains_subslice;
use rust_terminal::Terminal;

use crate::resources::{CARGO_COMMAND, setup_env};

#[test]
pub fn pre_commit() {
    let temp = setup_env();

    // Run command.
    let stderr = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stderr(CARGO_COMMAND, ["solar", "uninstall", "pre-commit"])
        .unwrap();

    assert!(contains_subslice(
        stderr.as_bytes(),
        " is not a git repository.".as_bytes()
    ));
}
