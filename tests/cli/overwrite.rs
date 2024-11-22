use crate::{cli, common};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn default_run_with_overwrite_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    // Adding empty files to tmp dir
    cli::create_files(
        tmp_dir_path,
        &[
            &format!("{}{}", cli::TEST_VYPER_CONTRACT_NAME, cli::BIN_EXTENSION),
            &format!(
                "{}{}",
                cli::TEST_VYPER_CONTRACT_NAME,
                cli::ERAVM_ASSEMBLY_EXTENSION
            ),
        ],
    );

    // Trying to run a command to get a warning and verify an exit code
    let pre_args = &[cli::TEST_VYPER_CONTRACT_PATH, "-o", tmp_dir_path];
    let pre_result = cli::execute_zkvyper(pre_args)?;
    pre_result
        .failure()
        .stderr(predicate::str::contains("Refusing to overwrite"));

    // Trying to add a flag and verify that command passed with 0 exit code
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-o",
        tmp_dir_path,
        "--overwrite",
        "-f",
        "eravm_assembly",
    ];
    let result = cli::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Refusing to overwrite").not());

    // Verify that files are not empty
    assert_eq!(
        false,
        cli::is_file_empty(&format!("{}/{}", tmp_dir_path, cli::VYPER_BIN_OUTPUT_NAME))?
    );

    Ok(())
}
