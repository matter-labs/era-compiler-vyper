//!
//! The Vyper compiler unit tests.
//!

pub mod r#const;

pub use self::r#const::*;

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;
use std::time::Duration;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;

use era_compiler_vyper::project::Project;
use era_compiler_vyper::vyper::standard_json::input::settings::optimize::Optimize as VyperStandardJsonInputSettingsOptimize;
use era_compiler_vyper::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use era_compiler_vyper::vyper::standard_json::input::Input as VyperStandardJsonInput;
use era_compiler_vyper::Build;
use era_compiler_vyper::VyperCompiler;

use crate::common;

/// Synchronization for vyper downloads.
static DOWNLOAD_VYPER: Once = Once::new();

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Download `vyper` binaries once
    DOWNLOAD_VYPER.call_once(|| {
        download_executables().expect("Unable to download vyper executables");
    });

    // Set the `zkvyper` binary path
    let zkvyper_bin = Command::cargo_bin(era_compiler_vyper::r#const::DEFAULT_EXECUTABLE_NAME)?;
    let _ = era_compiler_vyper::process::EXECUTABLE.set(PathBuf::from(zkvyper_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();
    Ok(())
}

///
/// Downloads the necessary compiler executables.
///
pub fn download_executables() -> anyhow::Result<()> {
    let mut http_client_builder = reqwest::blocking::ClientBuilder::new();
    http_client_builder = http_client_builder.connect_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.pool_idle_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.timeout(Duration::from_secs(60));
    let http_client = http_client_builder.build()?;

    let config_path = Path::new(VYPER_BIN_CONFIG);
    era_compiler_downloader::Downloader::new(http_client.clone()).download(config_path)?;

    // Copy the latest `vyper-*` binary to `vyper` for CLI tests
    let latest_vyper =
        PathBuf::from(get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable);
    let mut vyper = latest_vyper.clone();
    vyper.set_file_name(format!("vyper{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(latest_vyper, vyper)?;

    Ok(())
}

///
/// Returns the `vyper` compiler for the given version.
///
pub fn get_vyper_compiler(version: &semver::Version) -> anyhow::Result<VyperCompiler> {
    let vyper_path = PathBuf::from(VYPER_DOWNLOAD_DIR).join(format!(
        "{}-{}{}",
        VyperCompiler::DEFAULT_EXECUTABLE_NAME,
        version,
        std::env::consts::EXE_SUFFIX,
    ));

    VyperCompiler::new(vyper_path.to_str().expect("Always valid"))
}

///
/// Execute zkvyper with the given arguments and return the result.
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
/// Execute vyper with the given arguments and return the result.
///
pub fn execute_vyper(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let mut cmd = Command::new(vyper);
    Ok(cmd.args(args).assert())
}

///
/// Builds a test Vyper project via standard JSON.
///
pub fn build_vyper_standard_json(
    source_code: &str,
    version: &semver::Version,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<Build> {
    crate::common::setup()?;
    let vyper = crate::common::get_vyper_compiler(version)?;
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let mut sources = BTreeMap::new();
    sources.insert("test.vy".to_string(), source_code.to_string());
    let input = VyperStandardJsonInput::try_from_sources(
        sources.clone(),
        None,
        VyperStandardJsonInputSettingsSelection::generate_default(),
        VyperStandardJsonInputSettingsOptimize::None,
        vyper.version.default >= VyperCompiler::FIRST_VERSION_ENABLE_DECIMALS_SUPPORT,
        true,
        vec![],
    )?;

    let output = vyper.standard_json(input)?;

    let project = Project::try_from_standard_json(output, &vyper.version.default)?;
    let mut build = project.compile(
        None,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        vec![],
        None,
    )?;
    build.link(BTreeMap::new())?;
    Ok(build)
}

///
/// Builds a test Vyper project via combined JSON.
///
pub fn build_vyper_combined_json(
    input_paths: Vec<&str>,
    version: &semver::Version,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<Build> {
    crate::common::setup()?;
    let vyper = crate::common::get_vyper_compiler(version)?;
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let input_paths = input_paths.into_iter().map(PathBuf::from).collect();

    let project: Project = vyper.batch(
        &vyper.version.default,
        input_paths,
        &[],
        None,
        true,
        None,
        true,
    )?;

    let mut build = project.compile(
        None,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        vec![],
        None,
    )?;
    build.link(BTreeMap::new())?;
    Ok(build)
}

///
/// Checks if the specified `warning` was emitted during the `source_code` compilation.
///
pub fn check_warning(path: &str, version: &semver::Version, warning: &str) -> anyhow::Result<bool> {
    let build = build_vyper_combined_json(
        vec![path],
        version,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )?;
    for (_path, contract) in build.contracts.iter() {
        for contract_warning in contract.warnings.iter() {
            if contract_warning.message.contains(warning) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

///
/// Check if the file at the given path is empty.
///
pub fn is_file_empty(file_path: &str) -> anyhow::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() == 0)
}

///
/// Check if the output is the same as the file content.
///
pub fn is_output_same_as_file(file_path: &str, output: &str) -> anyhow::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content.trim().contains(output.trim()) || output.trim().contains(file_content.trim()))
}

///
/// Helper function to create files in a directory.
///
pub fn create_files(dir: &str, files: &[&str]) {
    for file in files {
        let path = Path::new(dir).join(Path::new(file));
        let mut file = File::create(path).expect("Failed to create file");
        writeln!(file, "").expect("Failed to write to file");
    }
}
