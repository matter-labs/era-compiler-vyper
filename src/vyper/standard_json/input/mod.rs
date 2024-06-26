//!
//! The `vyper --standard-json` input.
//!

pub mod language;
pub mod settings;
pub mod source;

use std::collections::BTreeMap;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Serialize;

use self::language::Language;
use self::settings::optimize::Optimize;
use self::settings::selection::Selection;
use self::settings::Settings;
use self::source::Source;

///
/// The `vyper --standard-json` input.
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The input language.
    pub language: Language,
    /// The input source code files hashmap.
    pub sources: BTreeMap<String, Source>,
    /// The compiler settings.
    pub settings: Settings,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn try_from_paths(
        language: Language,
        paths: &[PathBuf],
        evm_version: Option<era_compiler_common::EVMVersion>,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: Optimize,
        enable_decimals: bool,
        fallback_to_optimizing_for_size: bool,
        llvm_options: Vec<String>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_from(path.as_path()).unwrap_or_else(|error| {
                    panic!("Source code file {path:?} reading error: {error}")
                });
                (path.to_string_lossy().to_string(), source)
            })
            .collect();

        Ok(Self {
            language,
            sources,
            settings: Settings::new(
                evm_version,
                output_selection,
                optimize,
                enable_decimals,
                fallback_to_optimizing_for_size,
                llvm_options,
            ),
        })
    }

    ///
    /// A shortcut constructor.
    ///
    /// Only for the integration test purposes.
    ///
    pub fn try_from_sources(
        sources: BTreeMap<String, String>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: Optimize,
        enable_decimals: bool,
        fallback_to_optimizing_for_size: bool,
        llvm_options: Vec<String>,
    ) -> anyhow::Result<Self> {
        let sources = sources
            .into_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();

        Ok(Self {
            language: Language::Vyper,
            sources,
            settings: Settings::new(
                evm_version,
                output_selection,
                optimize,
                enable_decimals,
                fallback_to_optimizing_for_size,
                llvm_options,
            ),
        })
    }
}
