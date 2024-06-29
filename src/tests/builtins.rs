//!
//! The Vyper compiler unit tests for built-in functions.
//!

#![cfg(test)]

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

pub const SOURCE_CODE: &'static str = r#"
@external
def f():
    result: address = create_copy_of(convert(0x42, address))
    return
"#;

fn create_copy_of(version: semver::Version) {
    let _ = super::build_vyper(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
