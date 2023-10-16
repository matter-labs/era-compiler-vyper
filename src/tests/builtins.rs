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
        compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}

#[test]
#[should_panic(expected = "Built-in function `create_from_blueprint` is not supported")]
fn create_from_blueprint() {
    let source_code = r#"
@external
def f():
    result: address = create_from_blueprint(convert(0x42, address))
    return
"#;

    let _ = super::build_vyper(
        source_code,
        Some((
            semver::Version::new(0, 3, 10),
            "Built-in function `create_from_blueprint` is not supported",
        )),
        compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
