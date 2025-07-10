//!
//! The Vyper compiler unit test constants.
//!

/// Download directory for `vyper` binaries
pub const VYPER_DOWNLOAD_DIR: &'static str = "vyper-bin";

/// Path to the `vyper` binary configuration file
pub const VYPER_BIN_CONFIG: &'static str = "tests/vyper-bin.json";

/// LLVM IR file extension.
pub const LLVM_IR_EXTENSION: &'static str = ".ll";

/// Optimized LLVM IR file extension.
pub const LLVM_IR_OPTIMIZED_EXTENSION: &'static str = ".optimized.ll";

/// Unoptimized LLVM IR file extension.
pub const LLVM_IR_UNOPTIMIZED_EXTENSION: &'static str = ".unoptimized.ll";

/// EraVM assembly file extension.
pub const ERAVM_ASSEMBLY_EXTENSION: &'static str = ".zasm";

/// Binary output file extension.
pub const BIN_EXTENSION: &'static str = ".zbin";

/// A test input file.
pub const TEST_GREETER_CONTRACT_PATH: &'static str = "tests/data/contracts/vyper/greeter.vy";

/// A test input file.
pub const TEST_GREETER_CONTRACT_NAME: &'static str = "greeter.vy";

/// A test output file.
pub const VYPER_BIN_OUTPUT_NAME: &'static str = "greeter.vy.zbin";

/// A test output file.
pub const VYPER_ASM_OUTPUT_NAME: &'static str = "greeter.vy.zasm";

/// A test input file.
pub const TEST_TX_ORIGIN_CONTRACT_PATH: &'static str = "tests/data/contracts/vyper/tx_origin.vy";

/// A test input file.
pub const TEST_OPTIMIZER_CONTRACT_PATH: &'static str = "tests/data/contracts/vyper/optimizer.vy";

/// A test input file.
pub const TEST_SELFDESTRUCT_CONTRACT_PATH: &'static str =
    "tests/data/contracts/vyper/selfdestruct.vy";

/// A test input file.
pub const TEST_CREATE_COPY_OF_CONTRACT_PATH: &'static str =
    "tests/data/contracts/vyper/create_copy_of.vy";

/// A test input file.
pub const TEST_CREATE_MINIMAL_PROXY_TO_CONTRACT_PATH: &'static str =
    "tests/data/contracts/vyper/create_minimal_proxy_to.vy";

/// A test input file.
pub const TEST_RAW_CREATE_CONTRACT_PATH: &'static str = "tests/data/contracts/vyper/raw_create.vy";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_PATH: &'static str =
    "tests/data/contracts/eravm/default.zasm";

/// A test input file.
pub const TEST_LLVM_CONTRACT_PATH: &'static str = "tests/data/contracts/llvm/default.ll";

/// A test input file.
pub const TEST_JSON_CONTRACT_PATH: &'static str = "tests/data/contracts/json/default.json";

/// A test input file.
pub const TEST_BYTECODE_PATH: &'static str = "tests/data/bytecodes/default.zbin";
