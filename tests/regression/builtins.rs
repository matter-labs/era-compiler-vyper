//!
//! The Vyper compiler unit tests for built-in functions.
//!

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

fn create_copy_of(version: semver::Version) {
    let _ = super::build_vyper_combined_json(
        vec!["tests/regression/contracts/create_copy_of.vy"],
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
