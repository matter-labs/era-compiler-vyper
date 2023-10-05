//!
//! The Vyper contract AST.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::vyper::combined_json::contract::warning::Warning as CombinedJsonContractWarning;
use crate::warning_type::WarningType;

///
/// The Vyper contract AST.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct AST {
    /// The contract name.
    pub contract_name: String,
    /// The AST object.
    pub ast: serde_json::Value,
}

impl AST {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_name: String, ast: serde_json::Value) -> Self {
        Self { contract_name, ast }
    }

    ///
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(
        &self,
        ast: &serde_json::Value,
        suppressed_warnings: &[WarningType],
    ) -> Vec<CombinedJsonContractWarning> {
        let mut messages = Vec::new();
        if !suppressed_warnings.contains(&WarningType::EcRecover) {
            if let Some(message) = self.check_ecrecover(ast) {
                messages.push(message);
            }
        }
        if !suppressed_warnings.contains(&WarningType::ExtCodeSize) {
            if let Some(message) = self.check_extcodesize(ast) {
                messages.push(message);
            }
        }
        if !suppressed_warnings.contains(&WarningType::TxOrigin) {
            if let Some(message) = self.check_tx_origin(ast) {
                messages.push(message);
            }
        }

        match ast {
            serde_json::Value::Array(array) => {
                for element in array.iter() {
                    messages.extend(self.get_messages(element, suppressed_warnings));
                }
            }
            serde_json::Value::Object(object) => {
                for (_key, value) in object.iter() {
                    messages.extend(self.get_messages(value, suppressed_warnings));
                }
            }
            _ => {}
        }

        messages
    }

    ///
    /// Returns the code location as a string.
    ///
    pub fn location(&self, ast: &serde_json::Value) -> Option<(usize, usize)> {
        let line: usize = ast.get("lineno")?.as_u64()? as usize;
        let column: usize = ast.get("col_offset")?.as_u64()? as usize;
        Some((line, column))
    }

    ///
    /// Checks the AST node for the `ecrecover` function usage.
    ///
    pub fn check_ecrecover(&self, node: &serde_json::Value) -> Option<CombinedJsonContractWarning> {
        let ast = node.as_object()?;

        if ast.get("ast_type")?.as_str()? != "Call" {
            return None;
        }

        let function = ast.get("func")?.as_object()?;
        if function.get("ast_type")?.as_str()? != "Name" {
            return None;
        }
        if function.get("id")?.as_str()? != "ecrecover" {
            return None;
        }

        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are using 'ecrecover' to validate a signature of a user account.      │
│ zkSync Era comes with native account abstraction support, therefore it is highly recommended NOT │
│ to rely on the fact that the account has an ECDSA private key attached to it since accounts might│
│ implement other signature schemes.                                                               │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
        .to_owned();
        let (line, column) = self.location(node).unwrap_or((0, 0));
        Some(CombinedJsonContractWarning::new(
            self.contract_name.clone(),
            line,
            column,
            message,
        ))
    }

    ///
    /// Checks the AST node for the `extcodesize` value usage.
    ///
    pub fn check_extcodesize(
        &self,
        node: &serde_json::Value,
    ) -> Option<CombinedJsonContractWarning> {
        let ast = node.as_object()?;

        if ast.get("ast_type")?.as_str()? != "Attribute" {
            return None;
        }
        if ast.get("attr")?.as_str()? != "is_contract" {
            return None;
        }

        let value = ast.get("value")?.as_object()?;
        if value.get("ast_type")?.as_str()? != "Name" {
            return None;
        }

        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: Your code or one of its dependencies uses the 'extcodesize' instruction, which is       │
│ usually needed in the following cases:                                                           │
│   1. To detect whether an address belongs to a smart contract.                                   │
│   2. To detect whether the deploy code execution has finished.                                   │
│ zkSync Era comes with native account abstraction support (so accounts are smart contracts,       │
│ including private-key controlled EOAs), and you should avoid differentiating between contracts   │
│ and non-contract addresses.                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
        .to_owned();
        let (line, column) = self.location(node).unwrap_or((0, 0));
        Some(CombinedJsonContractWarning::new(
            self.contract_name.clone(),
            line,
            column,
            message,
        ))
    }

    ///
    /// Checks the AST node for the `tx.origin` value usage.
    ///
    pub fn check_tx_origin(&self, node: &serde_json::Value) -> Option<CombinedJsonContractWarning> {
        let ast = node.as_object()?;

        if ast.get("ast_type")?.as_str()? != "Attribute" {
            return None;
        }
        if ast.get("attr")?.as_str()? != "origin" {
            return None;
        }

        let value = ast.get("value")?.as_object()?;
        if value.get("ast_type")?.as_str()? != "Name" {
            return None;
        }
        if value.get("id")?.as_str()? != "tx" {
            return None;
        }

        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: You are checking for 'tx.origin' in your code, which might lead to unexpected behavior. │
│ zkSync Era comes with native account abstraction support, and therefore the initiator of a       │
│ transaction might be different from the contract calling your code. It is highly recommended NOT │
│ to rely on tx.origin, but use msg.sender instead.                                                │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();
        let (line, column) = self.location(node).unwrap_or((0, 0));
        Some(CombinedJsonContractWarning::new(
            self.contract_name.clone(),
            line,
            column,
            message,
        ))
    }
}
