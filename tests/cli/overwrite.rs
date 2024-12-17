use predicates::prelude::*;
use tempfile::TempDir;

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
        "-f",
        "eravm_assembly",
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
