//!
//! The `vyper --combined-json` contract warning.
//!

///
/// The contract.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Warning {
    /// The file path.
    pub file: String,
    /// The source code line.
    pub line: usize,
    /// The source code column.
    pub column: usize,
    /// The message text.
    pub message: String,
}

impl Warning {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(file: String, line: usize, column: usize, message: String) -> Self {
        Self {
            file,
            line,
            column,
            message,
        }
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\n ---> {}:{}:{}",
            self.message, self.file, self.line, self.column,
        )
    }
}
