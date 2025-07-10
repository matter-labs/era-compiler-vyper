//!
//! The Vyper compiler unit tests for unsupported opcodes.
//!
//! It is not possible to reproduce:
//! - PC
//! - CALLCODE
//! - EXTCODECOPY without using Vyper built-in functions forbidden on the AST level
//!

use crate::common;

#[cfg(not(target_arch = "aarch64"))]
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_3_3() {
    selfdestruct(semver::Version::new(0, 3, 3));
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_3_9() {
    selfdestruct(semver::Version::new(0, 3, 9));
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_3_10() {
    selfdestruct(semver::Version::new(0, 3, 10));
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_4_0() {
    selfdestruct(semver::Version::new(0, 4, 0));
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_4_1() {
    selfdestruct(semver::Version::new(0, 4, 1));
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_4_2() {
    selfdestruct(semver::Version::new(0, 4, 2));
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_0_4_3() {
    selfdestruct(semver::Version::new(0, 4, 3));
}

fn selfdestruct(version: semver::Version) {
    common::build_vyper_combined_json(
        vec![common::TEST_SELFDESTRUCT_CONTRACT_PATH],
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Test failure");
}
