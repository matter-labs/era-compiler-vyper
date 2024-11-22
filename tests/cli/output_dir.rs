use crate::{cli, common};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn default_run_with_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    // Check if output is empty and exit code
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "-o", tmp_dir_path_zk_vyper];
    let result = cli::execute_zkvyper(args)?;
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
        cli::is_file_empty(&format!(
            "{}/{}",
            tmp_dir_path_zk_vyper,
            cli::VYPER_BIN_OUTPUT_NAME
        ))?
    );

    Ok(())
}

#[test]
fn default_run_with_output_dir_and_assembly() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    // Check if output is empty and exit code
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-o",
        tmp_dir_path_zk_vyper,
        "-f",
        "eravm_assembly",
    ];
    let result = cli::execute_zkvyper(args)?;
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
        cli::is_file_empty(&format!(
            "{}/{}",
            tmp_dir_path_zk_vyper,
            cli::VYPER_BIN_OUTPUT_NAME
        ))?
    );
    assert_eq!(
        false,
        cli::is_file_empty(&format!(
            "{}/{}",
            tmp_dir_path_zk_vyper,
            cli::VYPER_ASM_OUTPUT_NAME
        ))?
    );

    Ok(())
}

#[test]
fn default_run_with_dual_output_dir_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zk_vyper = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path_zk_vyper = tmp_dir_zk_vyper.path().to_str().unwrap();

    // Check if dual output dir options results in an error
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-o",
        tmp_dir_path_zk_vyper,
        "-o",
        tmp_dir_path_zk_vyper,
    ];
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: the argument '--output-dir <OUTPUT_DIR>' cannot be used multiple times",
    ));

    Ok(())
}
