//!
//! Vyper compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// Pythonic Smart Contract Language for the EraVM.
///
/// Example: `zkvyper ERC20.vy`
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Vyper compiler for ZKsync",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Specify the input file paths.
    /// Multiple Vyper files can be passed in the default Vyper mode.
    /// LLVM IR mode currently supports only a single file.
    #[structopt(parse(from_os_str))]
    pub input_paths: Vec<PathBuf>,

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

    /// Try to recompile with -Oz if the bytecode is too large.
    #[structopt(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Pass arbitary space-separated options to LLVM.
    #[structopt(long = "llvm-options")]
    pub llvm_options: Option<String>,

    /// Disable the `vyper` LLL IR optimizer.
    #[structopt(long = "disable-vyper-optimizer")]
    pub disable_vyper_optimizer: bool,

    /// Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.
    /// In LLVM IR and EraVM assembly modes, `vyper` executable is unused.
    #[structopt(long = "vyper")]
    pub vyper: Option<String>,

    /// The EVM version to generate IR for.
    /// See https://github.com/matter-labs/era-compiler-common/blob/main/src/evm_version.rs for reference.
    #[structopt(long = "evm-version")]
    pub evm_version: Option<era_compiler_common::EVMVersion>,

    /// Enables decimals in the underlying `vyper` compiler.
    /// Only available in `vyper` v0.4.0 and later.
    #[structopt(long = "enable-decimals")]
    pub enable_decimals: bool,

    /// Set the output format selection.
    /// Available options: combined_json | ir_json | metadata | ast | abi | method_identifiers | layout | userdoc | devdoc | eravm_assembly
    #[structopt(short = "f")]
    pub format: Option<String>,

    /// Set the number of threads, which execute the tests concurrently.
    #[structopt(short = "t", long = "threads")]
    pub threads: Option<usize>,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined JSON mode.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Switch to EraVM assembly mode.
    /// Only one input EraVM assembly file is allowed.
    /// Cannot be used with combined JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[structopt(long = "eravm-assembly")]
    pub eravm_assembly: bool,

    /// Specify the bytecode file to disassemble.
    /// Two file types are allowed: raw binary bytecode (*.zbin), and hexadecimal string (*.hex).
    /// Cannot be used with combined and standard JSON modes.
    #[structopt(long = "disassemble")]
    pub disassemble: bool,

    /// Set the metadata hash type.
    /// Available types: `none`, `keccak256`, `ipfs`.
    /// The default is `keccak256`.
    #[structopt(long = "metadata-hash")]
    pub metadata_hash_type: Option<era_compiler_common::HashType>,

    /// Dump all IR (LLL, LLVM IR, assembly) to files in the specified directory.
    /// Only for testing and debugging.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `extcodesize`, `txorigin`.
    #[structopt(long = "suppress-warnings")]
    pub suppressed_warnings: Option<Vec<String>>,

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
            anyhow::bail!(
                "Error: No other options are allowed while getting the compiler version."
            );
        }

        if self.recursive_process && std::env::args().count() > 2 {
            anyhow::bail!("Error: No other options are allowed in recursive mode.");
        }

        let modes_count = [
            self.llvm_ir,
            self.eravm_assembly,
            self.disassemble,
            self.format.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if modes_count > 1 {
            anyhow::bail!(
                "Error: Only one modes is allowed at the same time: Vyper, LLVM IR, EraVM assembly, disassembler."
            );
        }

        if self.llvm_ir || self.eravm_assembly {
            if self.vyper.is_some() {
                anyhow::bail!("Error: `vyper` is not used in LLVM IR and EraVM assembly modes.");
            }

            if self.evm_version.is_some() {
                anyhow::bail!(
                    "Error: EVM version is not used in LLVM IR and EraVM assembly modes."
                );
            }
        }

        if self.eravm_assembly {
            if self.optimization.is_some() {
                anyhow::bail!(
                    "Error: LLVM optimizations are not supported in EraVM assembly mode."
                );
            }

            if self.fallback_to_optimizing_for_size {
                anyhow::bail!(
                    "Error: Falling back to -Oz is not supported in EraVM assembly mode."
                );
            }
        }

        if self.disassemble && std::env::args().count() > self.input_paths.len() + 2 {
            anyhow::bail!("Error: No other options are allowed in disassembler mode.");
        }

        Ok(())
    }

    ///
    /// Normalizes input paths by converting it to POSIX format.
    ///
    pub fn normalize_input_paths(&mut self) -> anyhow::Result<()> {
        for input_path in self.input_paths.iter_mut() {
            *input_path = era_compiler_vyper::path_to_posix(input_path.as_path())?;
        }
        Ok(())
    }
}
