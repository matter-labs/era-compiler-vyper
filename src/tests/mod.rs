//!
//! The Vyper compiler unit tests.
//!

#![cfg(test)]
#![allow(dead_code)]

pub mod builtins;
pub mod optimizer;
pub mod unsupported_opcodes;
pub mod warnings;

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::build::Build;
use crate::project::Project;
use crate::vyper::standard_json::input::settings::optimize::Optimize as VyperStandardJsonInputSettingsOptimize;
use crate::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use crate::vyper::standard_json::input::Input as VyperStandardJsonInput;
use crate::vyper::Compiler as VyperCompiler;

///
/// Builds a test Vyper project via standard JSON.
///
pub fn build_vyper_standard_json(
    source_code: &str,
    version: &semver::Version,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<Build> {
    check_dependencies();

    let vyper = VyperCompiler::new(
        format!(
            "{}-{version}{}",
            VyperCompiler::DEFAULT_EXECUTABLE_NAME,
            std::env::consts::EXE_SUFFIX
        )
        .as_str(),
    )?;

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

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
    let build = project.compile(
        None,
        true,
        optimizer_settings,
        vec![],
        zkevm_assembly::RunningVmEncodingMode::Production,
        vec![],
        None,
    )?;

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
    check_dependencies();

    let vyper = VyperCompiler::new(
        format!(
            "{}-{version}{}",
            VyperCompiler::DEFAULT_EXECUTABLE_NAME,
            std::env::consts::EXE_SUFFIX
        )
        .as_str(),
    )?;

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let input_paths = input_paths.into_iter().map(PathBuf::from).collect();

    let project: Project =
        vyper.batch(&vyper.version.default, input_paths, &[], None, true, true)?;

    let build = project.compile(
        None,
        true,
        optimizer_settings,
        vec![],
        zkevm_assembly::RunningVmEncodingMode::Production,
        vec![],
        None,
    )?;

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
/// Checks if the required tools are installed in the system.
///
fn check_dependencies() {
    for version in VyperCompiler::SUPPORTED_VERSIONS.into_iter() {
        let executable = format!(
            "{}-{version}{}",
            VyperCompiler::DEFAULT_EXECUTABLE_NAME,
            std::env::consts::EXE_SUFFIX
        );
        assert!(
            which::which(executable.as_str()).is_ok(),
            "The `{executable}` executable not found in ${{PATH}}"
        );
    }
}
