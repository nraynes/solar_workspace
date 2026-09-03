use std::path::Path;

use affirm_fs::DirStructure;
use clap::ValueEnum;
use rust_terminal::Terminal;
use solar_core::components::licenses::{LICENSES_DIR, license::License};

use crate::{component_tests::licenses::Snapshot, resources::CARGO_COMMAND};

mod cold_hot;

pub fn test_install_was_successful(
    path: &Path,
    include_licenses: Vec<&str>,
    licensed_under: Vec<&str>,
) {
    let mut command_args = vec!["solar", "install", "licenses"];
    let mut expected_include_licenses = DirStructure::new(path.join(LICENSES_DIR));
    let mut expected_licensed_under = DirStructure::new(path);

    let include_licenses_provided = !include_licenses.is_empty();
    let licensed_under_provided = !licensed_under.is_empty();

    // Add licenses from include licenses to command.
    if include_licenses_provided {
        command_args.push("--include-licenses");
        for license in include_licenses {
            command_args.push(license);
            expected_include_licenses = expected_include_licenses.file(
                License::from_str(license, true)
                    .unwrap()
                    .file_name()
                    .as_str(),
            );
        }
    }

    // Add licenses from licensed_under to command.
    if licensed_under_provided {
        command_args.push("--licensed-under");
        for license in licensed_under {
            command_args.push(license);
            expected_licensed_under = expected_licensed_under.file(
                License::from_str(license, true)
                    .unwrap()
                    .file_name()
                    .as_str(),
            );
        }
    }

    let expected_include_licenses = expected_include_licenses.build();
    let expected_licensed_under = expected_licensed_under.build();

    // Run command.
    Terminal::command()
        .current_dir(path)
        .piped()
        .run(CARGO_COMMAND, command_args)
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(path);

    if include_licenses_provided {
        let actual_include_licenses = snapshot_after.include_licenses().as_ref().unwrap();
        assert!(
            actual_include_licenses
                .contains()
                .structure(&expected_include_licenses)
        );
    }

    if licensed_under_provided {
        let actual_licensed_under = snapshot_after.licensed_under();
        assert!(
            actual_licensed_under
                .contains()
                .structure(&expected_licensed_under)
        );
    }
}
