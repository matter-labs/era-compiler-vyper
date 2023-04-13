//!
//! Vyper to zkEVM compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// Pythonic Smart Contract Language for the zkEVM.
///
/// Example: `zkvyper ERC20.vy`
///
#[derive(Debug, StructOpt)]
#[structopt(name = "The zkEVM Vyper compiler")]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Specify the input file paths.
    /// Multiple Vyper files can be passed in the default Vyper mode.
    /// LLVM IR mode currently supports only a single file.
    #[structopt(parse(from_os_str))]
    pub input_files: Vec<PathBuf>,

    /// Create one file per component and
    /// contract/file at the specified directory, if given.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[structopt(short = "O", long = "optimization")]
    pub optimization: Option<char>,

    /// Disable the `vyper` optimizer.
    #[structopt(long = "disable-vyper-optimizer")]
    pub disable_vyper_optimizer: bool,

    /// Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.
    /// LLVM IR mode: `vyper` is unused.
    #[structopt(long = "vyper")]
    pub vyper: Option<String>,

    /// The extra output format string.
    /// Passed to `vyper` without changes.
    #[structopt(short = "f")]
    pub format: Option<String>,

    /// Switch to the LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with the combined or standard JSON modes.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Dump all IRs to files in the specified directory.
    /// Only for testing and debugging.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Set the verify-each option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Set the debug-logging option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-debug-logging")]
    pub llvm_debug_logging: bool,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}
