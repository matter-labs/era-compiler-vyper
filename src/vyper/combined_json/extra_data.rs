//!
//! Extra data for combined JSON output.
//!

///
/// Extra data for combined JSON output.
///
#[derive(Debug, serde::Serialize)]
pub struct ExtraData {
    /// The project metadata.
    pub project_metadata: serde_json::Value,
}

impl ExtraData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(project_metadata: serde_json::Value) -> Self {
        Self { project_metadata }
    }
}
