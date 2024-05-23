//!
//! The Vyper compiler unit tests for unsupported opcodes.
//!
//! It is not possible to reproduce:
//! - PC
//! - CALLCODE
//!

#![cfg(test)]

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
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
