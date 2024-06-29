//!
//! The Vyper contract AST.
//!

use crate::message_type::MessageType;
use crate::vyper::combined_json::contract::warning::Warning as CombinedJsonContractWarning;

///
/// The Vyper contract AST.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
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
Warning: You are checking for 'tx.origin', which may lead to unexpected behavior.

ZKsync Era comes with native account abstraction support, and therefore the initiator of a
transaction might be different from the contract calling your code. It is highly recommended NOT
to rely on tx.origin, but use msg.sender instead.

Learn more about Account Abstraction at https://docs.zksync.io/build/developer-reference/account-abstraction/

You may disable this warning with `--suppress-warnings txorigin`.
"#
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
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(
        &self,
        ast: &serde_json::Value,
        suppressed_messages: &[MessageType],
    ) -> Vec<CombinedJsonContractWarning> {
        let mut messages = Vec::new();
        if !suppressed_messages.contains(&MessageType::TxOrigin) {
            if let Some(message) = self.check_tx_origin(ast) {
                messages.push(message);
            }
        }

        match ast {
            serde_json::Value::Array(array) => {
                for element in array.iter() {
                    messages.extend(self.get_messages(element, suppressed_messages));
                }
            }
            serde_json::Value::Object(object) => {
                for (_key, value) in object.iter() {
                    messages.extend(self.get_messages(value, suppressed_messages));
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
}
