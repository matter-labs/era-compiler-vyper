//!
//! The Vyper compiler unit tests for the optimizer.
//!

#![cfg(test)]

#[test]
fn default_0_3_3() {
    default(semver::Version::new(0, 3, 3));
}
#[test]
fn default_0_3_9() {
    default(semver::Version::new(0, 3, 9));
}
#[test]
fn default_0_3_10() {
    default(semver::Version::new(0, 3, 10));
}
#[test]
fn default_0_4_0() {
    default(semver::Version::new(0, 4, 0));
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

fn default(version: semver::Version) {
    let build_unoptimized = super::build_vyper(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = super::build_vyper(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = super::build_vyper(
        SOURCE_CODE,
        &version,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let size_when_unoptimized = build_unoptimized
        .contracts
        .get("test.vy:test")
        .expect("Missing file `test.vy:test`")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
        .contracts
        .get("test.vy:test")
        .expect("Missing file `test.vy:test`")
        .build
        .bytecode
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
        .contracts
        .get("test.vy:test")
        .expect("Missing file `test.vy:test`")
        .build
        .bytecode
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected the cycles-optimized bytecode to be smaller than the unoptimized. Optimized: {}B, Unoptimized: {}B", size_when_optimized_for_cycles, size_when_unoptimized,
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected the size-optimized bytecode to be smaller than the unoptimized. Optimized: {}B, Unoptimized: {}B", size_when_optimized_for_size, size_when_unoptimized,
    );
}
