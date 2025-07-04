//!
//! The Vyper compiler unit tests for built-in functions.
//!

use crate::common;

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of_0_3_10() {
    create_copy_of(semver::Version::new(0, 3, 10));
}
#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of_0_4_0() {
    create_copy_of(semver::Version::new(0, 4, 0));
}

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of_0_4_1() {
    create_copy_of(semver::Version::new(0, 4, 1));
}

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of_0_4_2() {
    create_copy_of(semver::Version::new(0, 4, 2));
}

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of_0_4_3() {
    create_copy_of(semver::Version::new(0, 4, 3));
}

fn create_copy_of(version: semver::Version) {
    let _ = common::build_vyper_combined_json(
        vec![common::TEST_CREATE_COPY_OF_CONTRACT_PATH],
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
