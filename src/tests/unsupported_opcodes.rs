//!
//! The Vyper compiler unit tests for unsupported opcodes.
//!
//! It is not possible to reproduce:
//! - PC
//! - CALLCODE
//! - EXTCODECOPY without using Vyper built-in functions forbidden on the AST level
//!

#![cfg(test)]

#[test]
#[should_panic(expected = "Built-in function `create_copy_of` is not supported")]
fn extcodecopy() {
    let source_code = r#"
@external
def f():
    result: address = create_copy_of(convert(0x42, address))
    return
"#;

    super::build_vyper(
        source_code,
        Some((
            semver::Version::new(0, 3, 9),
            "The `EXTCODECOPY` instruction is not supported",
        )),
        compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct() {
    let source_code = r#"
@external
def f():
    selfdestruct(convert(0x42, address))
"#;

    super::build_vyper(
        source_code,
        None,
        compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
