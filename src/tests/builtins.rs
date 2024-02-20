//!
//! The Vyper compiler unit tests for built-in functions.
//!

#![cfg(test)]

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn create_copy_of() {
    let source_code = r#"
@external
def f():
    result: address = create_copy_of(convert(0x42, address))
    return
"#;

    let _ = super::build_vyper(
        source_code,
        Some((
            semver::Version::new(0, 3, 10),
            "Built-in function `create_copy_of` is not supported",
        )),
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
