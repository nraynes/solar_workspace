use affirm_fs::DirStructure;
use rust_terminal::Terminal;
use solar_core::components::licenses::LICENSES_DIR;

use crate::{
    component_tests::licenses::{Snapshot, install::test_install_was_successful},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn licenses() {
    let temp = setup_env();

    test_install_was_successful(
        temp.root().path(),
        vec!["MIT", "Apache-2.0", "GPL-3.0", "X11", "eCos-2.0", "xpp"],
        vec!["MIT", "Apache-2.0", "GPL-3.0", "X11", "eCos-2.0", "xpp"],
    );

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            [
                "solar",
                "uninstall",
                "licenses",
                "--include-licenses",
                "MIT",
                "GPL-3.0",
                "--licensed-under",
                "X11",
                "xpp",
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let actual_include_licenses = snapshot_after.include_licenses().as_ref().unwrap();
    let actual_licensed_under = snapshot_after.licensed_under();

    let expected_include_licenses = DirStructure::new(temp.root().path().join(LICENSES_DIR))
        .file("LICENSE-Apache-2.0")
        .file("LICENSE-X11")
        .file("LICENSE-eCos-2.0")
        .file("LICENSE-xpp")
        .build();

    let expected_licensed_under = DirStructure::new(temp.root().path())
        .file("LICENSE-MIT")
        .file("LICENSE-GPL-3.0")
        .file("LICENSE-eCos-2.0")
        .file("LICENSE-Apache-2.0")
        .build();

    assert!(actual_include_licenses.eq().dir(&expected_include_licenses));
    assert!(actual_licensed_under.eq().dir(&expected_licensed_under));
}
