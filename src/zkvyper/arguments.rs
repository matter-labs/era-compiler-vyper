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

    /// The input file paths.
    #[structopt(parse(from_os_str))]
    pub input_files: Vec<PathBuf>,

    /// If given, creates one file per component and
    /// contract/file at the specified directory.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Disable the LLVM bytecode optimizer.
    #[structopt(long = "no-optimize")]
    pub no_optimize: bool,

    /// Sets the LLVM optimizer options.
    #[structopt(long = "llvm-opt")]
    pub llvm_options: Option<String>,

    /// Path to the `vyper` executable.
    /// By default, the one in $PATH is used.
    #[structopt(long = "vyper")]
    pub vyper: Option<String>,

    /// The extra output format string.
    #[structopt(short = "f")]
    pub format: Option<String>,

    /// Switch to the LLVM IR mode.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Dump all IRs to files in the specified directory.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,
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
