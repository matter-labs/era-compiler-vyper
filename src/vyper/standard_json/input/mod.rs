//!
//! The `vyper --standard-json` input representation.
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
use self::settings::evm_version::EVMVersion;
use self::settings::selection::Selection;
use self::settings::Settings;
use self::source::Source;

///
/// The `vyper --standard-json` input representation.
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
        evm_version: EVMVersion,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: bool,
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
            settings: Settings::new(evm_version, output_selection, optimize),
        })
    }

    ///
    /// A shortcut constructor.
    ///
    /// Only for the integration test purposes.
    ///
    pub fn try_from_sources(
        sources: BTreeMap<String, String>,
        evm_version: EVMVersion,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: bool,
    ) -> anyhow::Result<Self> {
        let sources = sources
            .into_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();

        Ok(Self {
            language: Language::Vyper,
            sources,
            settings: Settings::new(evm_version, output_selection, optimize),
        })
    }
}
