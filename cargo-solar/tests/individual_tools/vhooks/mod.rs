use std::{fs, path::PathBuf};

use rust_terminal::Terminal;
use solar_core::{Config, SOLARCONFIGNAME};

mod operations_default;
mod operations_moves_prior;
mod operations_with_arguments;
mod uninstall_no_config;
mod uninstall_no_vhooks;

pub fn git_hooks_path(path: PathBuf) -> String {
    let command_output = Terminal::command()
        .current_dir(path)
        .run("git", ["config", "core.hooksPath"])
        .unwrap();
    String::from_utf8(command_output.stdout).unwrap()
}

pub fn assert(input: bool, not: bool) {
    assert!(match not {
        true => !input,
        false => input,
    });
}

pub fn assert_eq<T>(x: T, y: T, not: bool)
where
    T: PartialEq + std::fmt::Debug,
{
    match not {
        true => assert_ne!(x, y),
        false => assert_eq!(x, y),
    }
}

pub fn assert_configuration(path: PathBuf, name: &str, remove_all: bool, not: bool) {
    let solar_config = Config::load_from_file(path.join(PathBuf::from(SOLARCONFIGNAME))).unwrap();
    let vhooks_config = solar_config.vhooks().as_ref().ok_or("").unwrap();
    assert_eq(vhooks_config.name(), &name.to_string(), not);
    assert_eq(vhooks_config.remove_all(), &remove_all, not);
}

pub fn assert_installation(path: PathBuf, name: &str, not: bool) {
    assert!(fs::exists(path.join(".git")).unwrap());
    assert(fs::exists(path.join(name)).unwrap(), not);
    assert_eq(git_hooks_path(path), format!("./{}\n", name), not);
}
