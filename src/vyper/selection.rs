//!
//! The `vyper` output selection flag.
//!

use std::str::FromStr;

///
/// The `vyper` output selection flag.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Selection {
    /// The combined JSON.
    CombinedJson,
    /// The stringified LLL IR.
    IR,
    /// The JSON LLL IR.
    IRJson,
    /// The metadata.
    Metadata,
    /// The AST.
    AST,
    /// The ABI data.
    ABI,
    /// The method identifiers.
    MethodIdentifiers,
}

impl FromStr for Selection {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> anyhow::Result<Self> {
        Ok(match string {
            "combined_json" => Self::CombinedJson,
            "ir" => Self::IR,
            "ir_json" => Self::IRJson,
            "metadata" => Self::Metadata,
            "ast" => Self::AST,
            "abi" => Self::ABI,
            "method_identifiers" => Self::MethodIdentifiers,
            _ => todo!(),
        })
    }
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CombinedJson => write!(f, "combined_json"),
            Self::IR => write!(f, "ir"),
            Self::IRJson => write!(f, "ir_json"),
            Self::Metadata => write!(f, "metadata"),
            Self::AST => write!(f, "ast"),
            Self::ABI => write!(f, "abi"),
            Self::MethodIdentifiers => write!(f, "method_identifiers"),
        }
    }
}
