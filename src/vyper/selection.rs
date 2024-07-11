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
    /// The storage layout.
    Layout,
    /// The interface.
    Interface,
    /// The external interface.
    ExternalInterface,
    /// The user documentation.
    UserDocumentation,
    /// The developer documentation.
    DeveloperDocumentation,
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
            "layout" => Self::Layout,
            "interface" => Self::Interface,
            "external_interface" => Self::ExternalInterface,
            "userdoc" => Self::UserDocumentation,
            "devdoc" => Self::DeveloperDocumentation,
            string => anyhow::bail!("Unknown selection flag `{string}`"),
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
            Self::Layout => write!(f, "layout"),
            Self::Interface => write!(f, "interface"),
            Self::ExternalInterface => write!(f, "external_interface"),
            Self::UserDocumentation => write!(f, "userdoc"),
            Self::DeveloperDocumentation => write!(f, "devdoc"),
        }
    }
}
