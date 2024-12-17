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

pub const SOURCE_CODE: &str = r#"
struct Todo:
    text: String[100]
    completed: bool

todos: public(Todo[100])
count: uint256

@external
def create_(_text: String[100]):
    # 2 ways to initialize a struct
    # key value mapping
    self.todos[self.count] = Todo({text: _text, completed: False})
    self.count += 1

    # initialize an empty struct and then update it
    todo: Todo = empty(Todo)
    todo.text = _text
    # todo.completed initialized to false

    self.todos[self.count] = todo
    self.count += 1

@external
@view
def get(_index: uint256) -> (String[100], bool):
    todo: Todo = self.todos[_index]
    return (todo.text, todo.completed)

@external
def update(_index: uint256, _text: String[100]):
    self.todos[_index].text = _text

@external
def toggleCompleted(_index: uint256):
    self.todos[_index].completed = not self.todos[_index].completed
"#;

fn default_standard_json(version: semver::Version) {
    let build_unoptimized = common::build_vyper_standard_json(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = common::build_vyper_standard_json(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = common::build_vyper_standard_json(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let size_when_unoptimized = build_unoptimized
        .contracts
        .get("test.vy:test")
        .expect("Missing contract `test.vy:test`")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
        .contracts
        .get("test.vy:test")
        .expect("Missing contract `test.vy:test`")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
        .contracts
        .get("test.vy:test")
        .expect("Missing contract `test.vy:test`")
        .build
        .bytecode
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected the cycles-optimized bytecode to be smaller than the unoptimized. Optimized: {}B, Unoptimized: {}B",
        size_when_optimized_for_cycles,
        size_when_unoptimized,
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected the size-optimized bytecode to be smaller than the unoptimized. Optimized: {}B, Unoptimized: {}B",
        size_when_optimized_for_size,
        size_when_unoptimized,
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
