use std::fmt;

use num_enum::IntoPrimitive;

use crate::models::account::PublicAccount;
use crate::models::errors::ERR_UNKNOWN_TYPE;
use std::collections::HashMap;

/// MultisigModificationTypeEnum :
/// The type of the modification:
/// * 0 - Add cosignatory.
/// * 1 - Remove cosignatory.
#[derive(Debug, Clone, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MultisigModificationType {
    Add,
    Remove
}

impl MultisigModificationType {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for MultisigModificationType {
    fn from(t: u8) -> Self {
        assert!(t <= 1, ERR_UNKNOWN_TYPE);
        match t {
            0 => MultisigModificationType::Add,
            _ => MultisigModificationType::Remove
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosignatoryModification {
    #[serde(rename = "modificationType")]
    pub modification_type: MultisigModificationType,
    /// The public key of the cosignatory account.
    #[serde(rename = "cosignatory_public_key")]
    pub public_account: PublicAccount,
}

impl CosignatoryModification {
    pub fn new(
        modification_type: MultisigModificationType,
        public_account: PublicAccount
    ) -> Self {
        CosignatoryModification { modification_type, public_account }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cosignature {
    /// The signature of the entity.
    /// The signature was generated by the signer and can be used to validate tha the entity data
    /// was not modified by a node.
    pub signature: String,
    /// The public account of the cosignatory.
    pub signer: PublicAccount,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultisigAccountInfo {
    /// The PublicAccount.
    #[serde(rename = "account")]
    pub account: PublicAccount,
    /// The number of signatures needed to approve a transaction.
    #[serde(rename = "minApproval")]
    pub min_approval: i32,
    /// The number of signatures needed to remove a cosignatory.
    #[serde(rename = "minRemoval")]
    pub min_removal: i32,
    /// The array of PublicAccount of the cosignatory accounts.
    #[serde(rename = "cosignatories")]
    pub cosignatories: Vec<PublicAccount>,
    /// The array of multisig accounts where the account is cosignatory.
    #[serde(rename = "multisigAccounts")]
    pub multisig_accounts: Vec<PublicAccount>,
}

impl fmt::Display for MultisigAccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultisigAccountGraphInfo {
    #[serde(rename = "MultisigAccounts")]
    pub multisig_accounts: HashMap<i16, Vec<MultisigAccountInfo>>,
}

impl fmt::Display for MultisigAccountGraphInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}