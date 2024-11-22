//!
//! Vyper compiler arguments.
//!

use std::path::PathBuf;

use clap::Parser;

///
/// Pythonic Smart Contract Language for the EraVM.
///
/// Example: `zkvyper ERC20.vy`
///
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long)]
    pub version: bool,

    /// Specify the input file paths.
    /// Multiple Vyper files can be passed in the default Vyper mode.
    /// LLVM IR mode currently supports only a single file.
    pub input_paths: Vec<PathBuf>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long)]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[arg(short = 'O', long)]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[arg(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Pass arbitary space-separated options to LLVM.
    /// The argument must be a single quoted string following a `=` separator.
    /// Example: `--llvm-options='-eravm-jump-table-density-threshold=10'`.
    #[arg(long)]
    pub llvm_options: Option<String>,

    /// Disable the `vyper` LLL IR optimizer.
    #[arg(long)]
    pub disable_vyper_optimizer: bool,

    /// Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.
    /// In LLVM IR and EraVM assembly modes, `vyper` executable is unused.
    #[arg(long)]
    pub vyper: Option<String>,

    /// The EVM version to generate IR for.
    /// See https://github.com/matter-labs/era-compiler-common/blob/main/era-compiler-common/src/evm_version.rs for reference.
    #[arg(long)]
    pub evm_version: Option<era_compiler_common::EVMVersion>,

    /// Enables decimals in the underlying `vyper` compiler.
    /// Only available in `vyper` v0.4.0 and later.
    #[arg(long)]
    pub enable_decimals: bool,

    /// Set the output format selection.
    /// Available options: combined_json | ir_json | metadata | ast | abi | method_identifiers | layout | userdoc | devdoc | eravm_assembly
    #[arg(short)]
    pub format: Option<String>,

    /// Set the number of threads, which execute the tests concurrently.
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined JSON mode.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[arg(long)]
    pub llvm_ir: bool,

    /// Switch to EraVM assembly mode.
    /// Only one input EraVM assembly file is allowed.
    /// Cannot be used with combined JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[arg(long)]
    pub eravm_assembly: bool,

    /// Specify the bytecode file to disassemble.
    /// Two file types are allowed: raw binary bytecode (*.zbin), and hexadecimal string (*.hex).
    /// Cannot be used with combined and standard JSON modes.
    #[arg(long)]
    pub disassemble: bool,

    /// Set the metadata hash type.
    /// Available types: `none`, `keccak256`, `ipfs`.
    /// The default is `keccak256`.
    #[arg(long)]
    pub metadata_hash: Option<era_compiler_common::HashType>,

    /// Dump all IR (LLL, LLVM IR, assembly) to files in the specified directory.
    /// Only for testing and debugging.
    #[arg(long)]
    pub debug_output_dir: Option<PathBuf>,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `extcodesize`, `txorigin`.
    #[arg(long)]
    pub suppress_warnings: Option<Vec<String>>,

    /// Set the `verify-each` option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_verify_each: bool,

    /// Set the `debug-logging` option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[arg(long)]
    pub recursive_process: bool,
}

impl Arguments {
    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.recursive_process {
            if std::env::args().count() > 2 {
                anyhow::bail!("Error: No other options are allowed in recursive mode.");
            } else {
                return Ok(());
            }
        }

        if self.version {
            if std::env::args().count() > 2 {
                anyhow::bail!(
                    "Error: No other options are allowed while getting the compiler version."
                );
            } else {
                return Ok(());
            }
        }

        if self.input_paths.is_empty() {
            anyhow::bail!("Error: No input files provided.");
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

        if self.disassemble && std::env::args().count() > self.input_paths.len() + 2 {
            anyhow::bail!("Error: No other options are allowed in disassembler mode.");
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
