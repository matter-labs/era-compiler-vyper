use tempfile::TempDir;

use era_compiler_vyper::VyperSelector;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure output directory is created
    assert!(tmp_dir_zkvyper.path().exists());

    // Ensure it contains expected filenames
    let expected_substrings = [
        common::LLVM_IR_EXTENSION,
        common::LLVM_IR_OPTIMIZED_EXTENSION,
        common::LLVM_IR_UNOPTIMIZED_EXTENSION,
        common::ERAVM_ASSEMBLY_EXTENSION,
    ];
    let filenames = std::fs::read_dir(tmp_dir_zkvyper.path())?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    assert!(filenames.iter().all(|filename| expected_substrings
        .iter()
        .any(|substring| filename.contains(substring))));

    Ok(())
}

#[test]
fn combined_json() -> anyhow::Result<()> {
    let _ = common::setup();

    let tmp_dir_zkvyper = TempDir::new()?;
    let selector = VyperSelector::CombinedJson.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-f",
        selector.as_str(),
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure output directory is created
    assert!(tmp_dir_zkvyper.path().exists());

    // Ensure it contains expected filenames
    let expected_substrings = [
        common::LLVM_IR_EXTENSION,
        common::LLVM_IR_OPTIMIZED_EXTENSION,
        common::LLVM_IR_UNOPTIMIZED_EXTENSION,
        common::ERAVM_ASSEMBLY_EXTENSION,
    ];
    let filenames = std::fs::read_dir(tmp_dir_zkvyper.path())?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    assert!(filenames.iter().all(|filename| expected_substrings
        .iter()
        .any(|substring| filename.contains(substring))));

    Ok(())
}
