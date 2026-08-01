// mod individual_tools;
// mod project_configs;

// use std::{env::current_dir, fmt::Debug, fs, path::Path};

// use mocked_up::TempEnv;
// use solar_core::{SOLARCONFIGNAME, sorted};

// pub fn copy_bin(path: &Path) {
//     let mut workspace = current_dir().unwrap();
//     workspace.pop();
//     fs::copy(
//         workspace.join("target/debug/cargo-solar"),
//         path.join("cargo-solar"),
//     )
//     .unwrap();
// }

// pub fn setup_env() -> TempEnv {
//     let mut temp = TempEnv::new().unwrap();
//     copy_bin(temp.env().path());
//     temp
// }

// pub fn assert(input: bool, assert_true: bool) {
//     assert!(match assert_true {
//         false => !input,
//         true => input,
//     });
// }

// pub fn assert_eq<T>(x: T, y: T, assert_true: bool)
// where
//     T: Eq + std::fmt::Debug,
// {
//     match assert_true {
//         false => assert_ne!(x, y),
//         true => assert_eq!(x, y),
//     }
// }

// pub fn assert_configuration_file_does_not_exist_at(path: &Path) {
//     assert!(!fs::exists(path.join(SOLARCONFIGNAME)).unwrap());
// }

// pub fn assert_vec_eq_unord<T>(vec_one: &Vec<T>, vec_two: &Vec<T>, assert_true: bool)
// where
//     T: Ord + Clone + Debug,
// {
//     assert_eq(
//         sorted(vec_one.clone()),
//         sorted(vec_two.clone()),
//         assert_true,
//     );
// }

// pub fn assert_opt_vec_eq_unord<T>(
//     opt_one: &Option<Vec<T>>,
//     opt_two: &Option<Vec<T>>,
//     assert_true: bool,
// ) where
//     T: Ord + Clone + Debug,
// {
//     if let Some(vec_one) = opt_one
//         && let Some(vec_two) = opt_two
//     {
//         assert_vec_eq_unord(vec_one, vec_two, assert_true);
//     } else {
//         assert_eq(opt_one, opt_two, assert_true);
//     }
// }
