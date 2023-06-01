//!
//! The Vyper compiler unit tests.
//!

#![cfg(test)]
#![allow(dead_code)]

pub mod builtins;

use std::collections::BTreeMap;

use crate::vyper::standard_json::input::settings::evm_version::EVMVersion as VyperStandardJsonInputSettingsEVMVersion;
use crate::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use crate::vyper::standard_json::input::Input as VyperStandardJsonInput;
use crate::vyper::Compiler as VyperCompiler;

pub fn build_vyper(source_code: &str, version: semver::Version) -> anyhow::Result<()> {
    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target();
    let optimizer_settings = compiler_llvm_context::OptimizerSettings::none();

    let mut sources = BTreeMap::new();
    sources.insert("test.vy".to_string(), source_code.to_string());
    let input = VyperStandardJsonInput::try_from_sources(
        sources.clone(),
        VyperStandardJsonInputSettingsEVMVersion::Paris,
        VyperStandardJsonInputSettingsSelection::generate_default(),
        true,
    )?;

    let vyper = VyperCompiler::new(VyperCompiler::DEFAULT_EXECUTABLE_NAME.to_owned());
    if vyper.version()?.default != version {
        return Ok(());
    }
    let output = vyper.standard_json(input)?;

    let project = output.try_into_project(&version)?;
    let _build = project.compile(
        compiler_llvm_context::TargetMachine::new(&optimizer_settings)?,
        optimizer_settings,
        false,
        None,
    )?;

    Ok(())
}
