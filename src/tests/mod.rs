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
use crate::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use crate::vyper::standard_json::input::Input as VyperStandardJsonInput;
use crate::vyper::Compiler as VyperCompiler;

///
/// Checks if the required tools are installed in the system.
///
fn check_dependencies() {
    for executable in [
        crate::r#const::DEFAULT_EXECUTABLE_NAME,
        VyperCompiler::DEFAULT_EXECUTABLE_NAME,
    ]
    .iter()
    {
        assert!(
            which::which(executable).is_ok(),
            "The `{executable}` executable not found in ${{PATH}}"
        );
    }
}

///
/// Builds a test Vyper project.
///
pub fn build_vyper(
    source_code: &str,
    version_filter: Option<(semver::Version, &str)>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<Build> {
    check_dependencies();

    let vyper = VyperCompiler::new(VyperCompiler::DEFAULT_EXECUTABLE_NAME)?;
    if let Some((version, message)) = version_filter {
        if vyper.version.default != version {
            panic!("{}", message);
        }
    }

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let mut sources = BTreeMap::new();
    sources.insert("test.vy".to_string(), source_code.to_string());
    let input = VyperStandardJsonInput::try_from_sources(
        sources.clone(),
        None,
        VyperStandardJsonInputSettingsSelection::generate_default(),
        true,
        true,
        vec![],
    )?;

    let output = vyper.standard_json(input)?;

    let project = output.try_into_project(&vyper.version.default)?;
    let build = project.compile(
        None,
        optimizer_settings,
        &[],
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        vec![],
        None,
    )?;

    Ok(build)
}

///
/// Checks if the specified `warning` was emitted during the `source_code` compilation.
///
pub fn check_warning(source_code: &str, warning: &str) -> anyhow::Result<bool> {
    let build = build_vyper(
        source_code,
        None,
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
