use assert_cmd::Command;
use era_compiler_vyper::VyperCompiler;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;

/// Synchronization for vyper downloads.
static DOWNLOAD_VYPER: Once = Once::new();

/// Download directory for `vyper` binaries
pub const VYPER_DOWNLOAD_DIR: &'static str = "vyper-bin";

/// Path to the `vyper` binary configuration file
pub const VYPER_BIN_CONFIG: &'static str = "tests/vyper-bin.json";

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

    VyperCompiler::new(vyper_path.to_str().unwrap())
}

///
/// Downloads the necessary compiler binaries.
///
pub fn download_binaries() -> anyhow::Result<()> {
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
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Download `vyper` binaries once
    DOWNLOAD_VYPER.call_once(|| {
        download_binaries().expect("Unable to download vyper binaries. Aborting...");
    });

    // Set the `zkvyper` binary path
    let zkvyper_bin = Command::cargo_bin(era_compiler_vyper::r#const::DEFAULT_EXECUTABLE_NAME)?;
    let _ = era_compiler_vyper::process::EXECUTABLE.set(PathBuf::from(zkvyper_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();
    Ok(())
}
