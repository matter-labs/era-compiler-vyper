use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

use era_compiler_vyper::VyperSelector;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    // Adding empty files to tmp dir
    common::create_files(
        tmp_dir_path,
        &[
            &format!(
                "{}{}",
                common::TEST_GREETER_CONTRACT_NAME,
                common::BIN_EXTENSION
            ),
            &format!(
                "{}{}",
                common::TEST_GREETER_CONTRACT_NAME,
                common::ERAVM_ASSEMBLY_EXTENSION
            ),
        ],
    );

    // Trying to run a command to get a warning and verify an exit code
    let pre_args = &[common::TEST_GREETER_CONTRACT_PATH, "-o", tmp_dir_path];
    let pre_result = common::execute_zkvyper(pre_args)?;
    pre_result
        .failure()
        .stderr(predicate::str::contains("Refusing to overwrite"));

    // Trying to add a flag and verify that command passed with 0 exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path,
        "--overwrite",
    ];
    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not());

    // Verify that files are not empty
    assert_eq!(
        false,
        common::is_file_empty(&format!("{tmp_dir_path}/{}", common::VYPER_BIN_OUTPUT_NAME))?
    );

    Ok(())
}

#[test]
fn all_output() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    // Adding empty files to tmp dir
    common::create_files(
        tmp_dir_path,
        &[
            &format!(
                "{}{}",
                common::TEST_GREETER_CONTRACT_NAME,
                common::BIN_EXTENSION
            ),
            &format!(
                "{}{}",
                common::TEST_GREETER_CONTRACT_NAME,
                common::ERAVM_ASSEMBLY_EXTENSION
            ),
        ],
    );

    // Trying to run a command to get a warning and verify an exit code
    let pre_args = &[common::TEST_GREETER_CONTRACT_PATH, "-o", tmp_dir_path];
    let pre_result = common::execute_zkvyper(pre_args)?;
    pre_result
        .failure()
        .stderr(predicate::str::contains("Refusing to overwrite"));

    let format = [
        VyperSelector::IRJson,
        VyperSelector::AST,
        VyperSelector::ABI,
        VyperSelector::MethodIdentifiers,
        VyperSelector::Layout,
        VyperSelector::UserDocumentation,
        VyperSelector::DeveloperDocumentation,
        VyperSelector::EraVMAssembly,
        VyperSelector::ProjectMetadata,
    ]
    .into_iter()
    .map(|selection| selection.to_string())
    .collect::<Vec<String>>()
    .join(",");

    // Trying to add a flag and verify that command passed with 0 exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path,
        "-f",
        format.as_str(),
        "--overwrite",
    ];
    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not());

    // Verify that files are not empty
    assert_eq!(
        false,
        common::is_file_empty(&format!("{tmp_dir_path}/{}", common::VYPER_BIN_OUTPUT_NAME))?
    );
    assert_eq!(
        false,
        common::is_file_empty(&format!("{tmp_dir_path}/{}", common::VYPER_ASM_OUTPUT_NAME))?
    );
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path}/{}",
            common::TEST_GREETER_CONTRACT_NAME
        ))?
    );

    Ok(())
}

#[test_case(VyperSelector::CombinedJson)]
#[test_case(VyperSelector::IRJson)]
#[test_case(VyperSelector::AST)]
#[test_case(VyperSelector::ABI)]
#[test_case(VyperSelector::MethodIdentifiers)]
#[test_case(VyperSelector::Layout)]
#[test_case(VyperSelector::UserDocumentation)]
#[test_case(VyperSelector::DeveloperDocumentation)]
#[test_case(VyperSelector::EraVMAssembly)]
#[test_case(VyperSelector::ProjectMetadata)]
fn one_output_not_passed(selector: VyperSelector) -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    let selector = selector.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path,
        "-f",
        selector.as_str(),
    ];
    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not());
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Refusing to overwrite"));

    Ok(())
}

#[test]
fn all_output_not_passed() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    let format = [
        VyperSelector::IRJson,
        VyperSelector::AST,
        VyperSelector::ABI,
        VyperSelector::MethodIdentifiers,
        VyperSelector::Layout,
        VyperSelector::UserDocumentation,
        VyperSelector::DeveloperDocumentation,
        VyperSelector::EraVMAssembly,
        VyperSelector::ProjectMetadata,
    ]
    .into_iter()
    .map(|selection| selection.to_string())
    .collect::<Vec<String>>()
    .join(",");

    // Trying to add a flag and verify that command passed with 0 exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path,
        "-f",
        format.as_str(),
    ];
    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not());
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Refusing to overwrite"));

    Ok(())
}
