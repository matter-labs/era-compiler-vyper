use predicates::prelude::*;
use tempfile::TempDir;

use era_compiler_vyper::VyperSelection;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    // Check if output is empty and exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path_zk_vyper,
    ];
    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not())
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Verify output directory and file creation
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::VYPER_BIN_OUTPUT_NAME
        ))?
    );

    Ok(())
}

#[test]
fn combined_json() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-f",
        "combined_json",
        "-o",
        tmp_dir_path_zk_vyper,
    ];

    let result = common::execute_zkvyper(args)?;
    result.success();

    assert!(
        std::fs::exists(format!(
            "{tmp_dir_path_zk_vyper}/combined.{}",
            era_compiler_common::EXTENSION_JSON,
        ))
        .expect("Always valid"),
        "Combined JSON file not found"
    );

    Ok(())
}

#[test]
fn eravm_assembly_only() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    // Check if output is empty and exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path_zk_vyper,
        "-f",
        "eravm_assembly",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not())
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Verify output directory and file creation
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::VYPER_BIN_OUTPUT_NAME
        ))?
    );
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::VYPER_ASM_OUTPUT_NAME
        ))?
    );

    Ok(())
}

#[test]
fn all_output() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    let format = [
        VyperSelection::IRJson,
        VyperSelection::AST,
        VyperSelection::ABI,
        VyperSelection::MethodIdentifiers,
        VyperSelection::Layout,
        VyperSelection::UserDocumentation,
        VyperSelection::DeveloperDocumentation,
        VyperSelection::EraVMAssembly,
        VyperSelection::ProjectMetadata,
    ]
    .into_iter()
    .map(|selection| selection.to_string())
    .collect::<Vec<String>>()
    .join(",");

    // Check if output is empty and exit code
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-o",
        tmp_dir_path_zk_vyper,
        "-f",
        format.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not())
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Verify output directory and file creation
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::VYPER_BIN_OUTPUT_NAME
        ))?
    );
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::VYPER_ASM_OUTPUT_NAME
        ))?
    );
    assert_eq!(
        false,
        common::is_file_empty(&format!(
            "{tmp_dir_path_zk_vyper}/{}",
            common::TEST_GREETER_CONTRACT_NAME
        ))?
    );

    Ok(())
}
