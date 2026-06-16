use std::{fs, path::Path};

use solar_core::{
    Config,
    tool::{Parameters, Workflow},
};

mod double_install;
mod install_no_args;
mod operations_default;
mod overwrite_workflow;
mod uninstall_no_install;

use crate::{assert, assert_opt_vec_eq_unord};

pub fn assert_configuration(path: &Path, expected_workflows_list: Option<Vec<Workflow>>) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let github_workflows_config = solar_config.github_workflows().as_ref().unwrap();
    let actual_workflows_list = github_workflows_config.workflows_list().clone();
    assert_opt_vec_eq_unord(&actual_workflows_list, &expected_workflows_list, true);
}

pub fn assert_installation(path: &Path, expected_workflows_list: Option<Vec<Workflow>>) {
    println!("Checking workflow directory existence.");
    let workflows_path = path.join(".github/workflows");
    assert(
        fs::exists(&workflows_path).unwrap(),
        expected_workflows_list.is_some(),
    );
    if let Some(workflows_list) = expected_workflows_list {
        for workflow in workflows_list {
            println!(
                "Checking existence of workflow {}.",
                workflow.get().file().name()
            );
            let workflow_path = path.join(workflows_path.join(workflow.get().file().name()));
            assert!(fs::exists(&workflow_path).unwrap());
            println!(
                "Checking that workflow {} matches expected content.",
                workflow.get().file().name()
            );
            let parameters = Parameters::new(
                path.canonicalize()
                    .unwrap()
                    .file_name()
                    .ok_or("Could not get name of working directory")
                    .unwrap()
                    .to_str()
                    .ok_or("Could not convert directory name to string.")
                    .unwrap()
                    .to_string(),
            );
            let workflow_content = fs::read_to_string(workflow_path).unwrap();
            assert_eq!(workflow_content, workflow.get().get(&parameters));
        }
    }
}
