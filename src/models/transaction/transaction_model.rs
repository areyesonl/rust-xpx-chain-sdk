use std::fmt;

use failure::_core::fmt::Debug;

use crate::models::transaction::AbstractTransaction;
use crate::models::account::Account;

#[derive(Debug, PartialEq, Serialize)]
pub struct TransactionPayload {
    /// The transaction payload.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl TransactionPayload {
    pub fn new() -> TransactionPayload {
        TransactionPayload {
            payload: None,
        }
    }
}

pub trait Transaction: Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn get_abs_transaction(self) -> AbstractTransaction;

    fn size(&self) -> usize;

    /// An abstract method to generate the transaction bytes.
    fn generate_bytes(&self) -> Vec<u8>;

    /// An abstract method to generate the embedded transaction bytes.
    fn generate_embedded_bytes(&self) -> Vec<u8>;

    /// Serialize this transaction object.
    fn serialize(&self) -> String;

    /// Returns `true` if this transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool;

    fn sign_with(&self, account: Account, generation_hash: String);
}

serialize_trait_object!(Transaction);

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        &self.generate_bytes() == &other.generate_bytes()
    }
}

impl fmt::Display for dyn Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}