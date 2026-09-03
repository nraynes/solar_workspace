use affirm_fs::contains_subslice;
use rust_terminal::Terminal;

use crate::resources::{CARGO_COMMAND, setup_env};

#[test]
pub fn commitalyzer() {
    let temp = setup_env();

    let stderr = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stderr(CARGO_COMMAND, ["solar", "install", "commitalyzer"])
        .unwrap();

    assert!(contains_subslice(
        stderr.as_bytes(),
        " is not a git repository.".as_bytes()
    ));
}
