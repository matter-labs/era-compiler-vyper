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

    /// Create one file per component and contract/file at the specified directory, if given.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[structopt(short = "O", long = "optimization")]
    pub optimization: Option<char>,

    /// Disable the `vyper` LLL IR optimizer.
    #[structopt(long = "disable-vyper-optimizer")]
    pub disable_vyper_optimizer: bool,

    /// Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.
    /// In LLVM IR mode `vyper` is unused.
    #[structopt(long = "vyper")]
    pub vyper: Option<String>,

    /// An extra output format string.
    /// See `vyper --help` for available options.
    #[structopt(short = "f")]
    pub format: Option<String>,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined or standard JSON modes.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Switch to zkEVM assembly mode.
    /// Only one input zkEVM assembly file is allowed.
    /// Cannot be used with combined or standard JSON modes.
    #[structopt(long = "zkasm")]
    pub zkasm: bool,

    /// Set metadata hash mode: `keccak256` | `none`.
    /// `keccak256` is enabled by default.
    #[structopt(long = "metadata-hash")]
    pub metadata_hash: Option<String>,

    /// Dump all IR (LLL, LLVM IR, assembly) to files in the specified directory.
    /// Only for testing and debugging.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Set the `verify-each` option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Set the `debug-logging` option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-debug-logging")]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[structopt(long = "recursive-process")]
    pub recursive_process: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }

    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed while getting the compiler version.");
        }

        if self.recursive_process && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed in recursive mode.");
        }

        if self.llvm_ir && self.zkasm {
            anyhow::bail!("Either LLVM IR or assembly mode can be used, but not both.");
        }

        Ok(())
    }
}
