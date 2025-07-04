//!
//! The Vyper compiler unit tests for the optimizer.
//!

use crate::common;

#[cfg(not(target_arch = "aarch64"))]
#[test]
fn default_standard_json_0_3_3() {
    default_standard_json(semver::Version::new(0, 3, 3));
}
#[test]
fn default_standard_json_0_3_9() {
    default_standard_json(semver::Version::new(0, 3, 9));
}
#[test]
fn default_standard_json_0_3_10() {
    default_standard_json(semver::Version::new(0, 3, 10));
}
#[test]
fn default_standard_json_0_4_0() {
    default_standard_json(semver::Version::new(0, 4, 0));
}

#[test]
fn default_standard_json_0_4_1() {
    default_standard_json(semver::Version::new(0, 4, 1));
}

#[test]
fn default_standard_json_0_4_2() {
    default_standard_json(semver::Version::new(0, 4, 2));
}

#[test]
fn default_standard_json_0_4_3() {
    default_standard_json(semver::Version::new(0, 4, 3));
}

#[cfg(not(target_arch = "aarch64"))]
#[test]
fn default_combined_json_0_3_3() {
    default_combined_json(semver::Version::new(0, 3, 3));
}
#[test]
fn default_combined_json_0_3_9() {
    default_combined_json(semver::Version::new(0, 3, 9));
}
#[test]
fn default_combined_json_0_3_10() {
    default_combined_json(semver::Version::new(0, 3, 10));
}
#[test]
fn default_combined_json_0_4_0() {
    default_combined_json(semver::Version::new(0, 4, 0));
}

#[test]
fn default_combined_json_0_4_1() {
    default_combined_json(semver::Version::new(0, 4, 1));
}

#[test]
fn default_combined_json_0_4_2() {
    default_combined_json(semver::Version::new(0, 4, 2));
}

#[test]
fn default_combined_json_0_4_3() {
    default_combined_json(semver::Version::new(0, 4, 3));
}

fn default_standard_json(version: semver::Version) {
    let sources = common::read_sources(&[common::TEST_OPTIMIZER_CONTRACT_PATH]);

    let build_unoptimized = common::build_vyper_standard_json(
        sources.clone(),
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = common::build_vyper_standard_json(
        sources.clone(),
        &version,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = common::build_vyper_standard_json(
        sources,
        &version,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let full_path = format!("{}:optimizer", common::TEST_OPTIMIZER_CONTRACT_PATH);

    let size_when_unoptimized = build_unoptimized
        .contracts
        .get(full_path.as_str())
        .expect("Missing contract")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
        .contracts
        .get(full_path.as_str())
        .expect("Missing contract")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
        .contracts
        .get(full_path.as_str())
        .expect("Missing contract")
        .build
        .bytecode
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected the cycles-optimized bytecode to be smaller than the unoptimized. Optimized: {size_when_optimized_for_cycles}B, Unoptimized: {size_when_unoptimized}B",
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected the size-optimized bytecode to be smaller than the unoptimized. Optimized: {size_when_optimized_for_size}B, Unoptimized: {size_when_unoptimized}B",
    );
}

fn default_combined_json(version: semver::Version) {
    let build_unoptimized = common::build_vyper_combined_json(
        vec![common::TEST_OPTIMIZER_CONTRACT_PATH],
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = common::build_vyper_combined_json(
        vec![common::TEST_OPTIMIZER_CONTRACT_PATH],
        &version,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = common::build_vyper_combined_json(
        vec![common::TEST_OPTIMIZER_CONTRACT_PATH],
        &version,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let size_when_unoptimized = build_unoptimized
        .contracts
        .get(common::TEST_OPTIMIZER_CONTRACT_PATH)
        .expect("Missing contract")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
        .contracts
        .get(common::TEST_OPTIMIZER_CONTRACT_PATH)
        .expect("Missing contract")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
        .contracts
        .get(common::TEST_OPTIMIZER_CONTRACT_PATH)
        .expect("Missing contract")
        .build
        .bytecode
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected the cycles-optimized bytecode to be smaller than the unoptimized. Optimized: {size_when_optimized_for_cycles}B, Unoptimized: {size_when_unoptimized}B",
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected the size-optimized bytecode to be smaller than the unoptimized. Optimized: {size_when_optimized_for_size}B, Unoptimized: {size_when_unoptimized}B",
    );
}
