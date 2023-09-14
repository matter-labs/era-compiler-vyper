//!
//! The Vyper compiler unit tests.
//!

#![cfg(test)]
#![allow(dead_code)]

pub mod builtins;

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::vyper::standard_json::input::settings::evm_version::EVMVersion as VyperStandardJsonInputSettingsEVMVersion;
use crate::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use crate::vyper::standard_json::input::Input as VyperStandardJsonInput;
use crate::vyper::Compiler as VyperCompiler;

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

pub fn build_vyper(
    source_code: &str,
    version: semver::Version,
    message_version: &str,
) -> anyhow::Result<()> {
    check_dependencies();

    let vyper = VyperCompiler::new(VyperCompiler::DEFAULT_EXECUTABLE_NAME.to_owned());
    if vyper.version.default != version {
        panic!("{}", message_version);
    }

    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target(compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));
    let optimizer_settings = compiler_llvm_context::OptimizerSettings::none();

    let mut sources = BTreeMap::new();
    sources.insert("test.vy".to_string(), source_code.to_string());
    let input = VyperStandardJsonInput::try_from_sources(
        sources.clone(),
        VyperStandardJsonInputSettingsEVMVersion::Paris,
        VyperStandardJsonInputSettingsSelection::generate_default(),
        true,
    )?;

    let output = vyper.standard_json(input)?;

    let project = output.try_into_project(&vyper.version.default)?;
    let _build = project.compile(
        optimizer_settings,
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;

    Ok(())
}
