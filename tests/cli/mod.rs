use crate::common;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod basic;
mod debug_output_dir;
mod disable_vyper_optimizer;
mod disassembler;
mod eravm_assembly;
mod fallback;
mod format;
mod llvm_debug_log;
mod llvm_ir;
mod llvm_options;
mod llvm_verify_each;
mod metadata_hash;
mod optimization;
mod output_dir;
mod overwrite;
mod supress;
mod version;
mod vyper;

/// The solidity contract name
pub const TEST_VYPER_CONTRACT_NAME: &'static str = "contract.vy";
/// The solidity contract full path
pub const TEST_VYPER_CONTRACT_PATH: &'static str = "tests/cli/contracts/vyper/contract.vy";

pub const TEST_TX_ORIGIN_CONTRACT_PATH: &'static str = "tests/cli/contracts/vyper/txorigin.vy";

/// The solidity binary artifact output name
pub const VYPER_BIN_OUTPUT_NAME: &'static str = "contract.vy.zbin";

/// The solidity assembly artifact output name
pub const VYPER_ASM_OUTPUT_NAME: &'static str = "contract.vy.zasm";

/// The era assembly contract path
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_PATH: &'static str =
    "tests/cli/contracts/eravm/contract.zasm";

/// The LLVM contract path
pub const TEST_LLVM_CONTRACT_PATH: &'static str = "tests/cli/contracts/llvm/contract.ll";

/// The standard JSON contract path
pub const TEST_JSON_CONTRACT_PATH: &'static str = "tests/cli/contracts/json/contract.json";

/// The binary bytecode sample path
pub const TEST_BINARY_BYTECODE_PATH: &'static str = "tests/cli/bytecodes/bytecode.zbin";

/// The hexadecimal string bytecode sample path
pub const TEST_HEXADECIMAL_BYTECODE_PATH: &'static str = "tests/cli/bytecodes/bytecode.hex";

/// LLVM IR file extension
pub const LLVM_IR_EXTENSION: &'static str = ".ll";

/// Optimized LLVM IR file extension
pub const LLVM_IR_OPTIMIZED_EXTENSION: &'static str = ".optimized.ll";

/// Unoptimized LLVM IR file extension
pub const LLVM_IR_UNOPTIMIZED_EXTENSION: &'static str = ".unoptimized.ll";

/// EraVM assembly file extension
pub const ERAVM_ASSEMBLY_EXTENSION: &'static str = ".zasm";

/// Binary output file extension
pub const BIN_EXTENSION: &'static str = ".zbin";

///
/// Execute zkvyper with the given arguments and return the result
///
pub fn execute_zkvyper(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin(era_compiler_vyper::r#const::DEFAULT_EXECUTABLE_NAME)?;
    Ok(cmd
        .env(
            "PATH",
            fs::canonicalize(&PathBuf::from(common::VYPER_DOWNLOAD_DIR))?,
        )
        .args(args)
        .assert())
}

///
/// Execute vyper with the given arguments and return the result
///
pub fn execute_vyper(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let mut cmd = Command::new(vyper);
    Ok(cmd.args(args).assert())
}

///
/// Check if the file at the given path is empty
///
pub fn is_file_empty(file_path: &str) -> anyhow::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() == 0)
}

///
/// Check if the output is the same as the file content
///
pub fn is_output_same_as_file(file_path: &str, output: &str) -> anyhow::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content.trim().contains(output.trim()) || output.trim().contains(file_content.trim()))
}

///
/// Helper function to create files in a directory
///
fn create_files(dir: &str, files: &[&str]) {
    for file in files {
        let path = Path::new(dir).join(Path::new(file));
        let mut file = File::create(path).expect("Failed to create file");
        writeln!(file, "").expect("Failed to write to file");
    }
}
