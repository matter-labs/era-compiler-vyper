mod builtins;
mod optimizer;
mod unsupported_opcodes;
mod warnings;

use era_compiler_vyper::project::Project;
use era_compiler_vyper::vyper::standard_json::input::settings::optimize::Optimize as VyperStandardJsonInputSettingsOptimize;
use era_compiler_vyper::vyper::standard_json::input::settings::selection::Selection as VyperStandardJsonInputSettingsSelection;
use era_compiler_vyper::vyper::standard_json::input::Input as VyperStandardJsonInput;
use era_compiler_vyper::{Build, VyperCompiler};
use std::collections::BTreeMap;
use std::path::PathBuf;

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
    let build = project.compile(None, true, optimizer_settings, vec![], vec![], None)?;

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

    let project: Project =
        vyper.batch(&vyper.version.default, input_paths, &[], None, true, true)?;

    let build = project.compile(None, true, optimizer_settings, vec![], vec![], None)?;

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
