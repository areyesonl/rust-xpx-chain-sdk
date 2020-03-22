use std::collections::HashMap;

use fb::FlatBufferBuilder;

use crate::models::account::PublicAccount;
use crate::models::consts::{SIGNATURE_SIZE, SIGNER_SIZE};
use crate::models::network::NetworkType;
use crate::models::Uint64;

use super::{
    deadline::Deadline,
    EntityTypeEnum,
    EntityVersion,
    Hash,
    Height
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) transaction_info: Option<TransactionInfo>,

    pub(crate) network_type: NetworkType,

    /// The signature was generated by the signer and can be used to validate tha the entity
    /// data was not modified by a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) signature: Option<String>,

    /// The 'PublicAccount' of the entity signer formatted as hexadecimal.
    pub(crate) signer: PublicAccount,

    /// The transaction version.
    pub(crate) version: EntityVersion,

    /// The transaction type.
    #[serde(rename = "type")]
    pub(crate) transaction_type: EntityTypeEnum,

    /// The maximum fee allowed to be spent for this transaction.
    ///
    /// The higher the fee, the higher the priority of the transaction. Transactions with high
    /// priority get included in a block before transactions with lower priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_fee: Option<Uint64>,

    /// The 'Deadline' for the transaction to be included in a block before it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) deadline: Option<Deadline>,
}

impl AbstractTransaction {
    pub fn new(tx_info: Option<TransactionInfo>,
               network_type: NetworkType,
               signature: Option<String>,
               signer: PublicAccount,
               version: EntityVersion,
               transaction_type: EntityTypeEnum,
               max_fee: Option<Uint64>,
               deadline: Option<Deadline>
    ) -> Self {
        AbstractTransaction {
            transaction_info: tx_info,
            network_type,
            signature,
            signer,
            version,
            transaction_type,
            max_fee,
            deadline,
        }
    }

    pub(crate) fn get_hash(&self) -> Hash {
        match self.transaction_info.to_owned() {
            Some(h) => {
                let hash = match h.transaction_hash {
                    Some(hs) => hs,
                    _ => "".to_string()
                };
                hash
            },
            _ => "".to_string()
        }
    }

    pub fn new_from_type(deadline: Deadline,
                         version: EntityVersion,
                         transaction_type: EntityTypeEnum, network_type: NetworkType) -> Self {
        AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: None,
            signer: Default::default(),
            version,
            transaction_type,
            max_fee: None,
            deadline: Some(deadline)
        }
    }

    pub(crate) fn is_unconfirmed(&self) -> bool {
        return if let Some(tx_info) = &self.transaction_info {
            tx_info.height.0.to_owned() == 0 &&
                tx_info.transaction_hash.eq(&tx_info.merkle_component_hash)
        } else {
            false
        };
    }

    pub(crate) fn is_confirmed(&self) -> bool {
        return if let Some(tx_info) = &self.transaction_info {
            tx_info.height.0 > 0
        } else {
            false
        };
    }

    pub(crate) fn has_missing_signatures(&self) -> bool {
        return if let Some(tx_info) = &self.transaction_info {
            tx_info.height.0 == 0 && tx_info.transaction_hash.eq(
                &tx_info.merkle_component_hash
            )
        } else {
            false
        };
    }

    pub(crate) fn is_unannounced(&self) -> bool {
        unimplemented!()
    }

    pub(crate) fn to_aggregate(&mut self, signer: PublicAccount) {
        self.signer = signer;
    }

    pub(crate) fn generate_vector(&self, builder: &mut FlatBufferBuilder) -> HashMap<&str, fb::UOffsetT> {
        let mut data: HashMap<&str, fb::UOffsetT> = HashMap::new();

        let max_fee = match self.max_fee {
            Some(item) => item,
            _ => Uint64::default()
        };

        let deadline = match self.deadline {
            Some(item) => item,
            _ => Deadline::default()
        };

        let network_type: fb::UOffsetT = self.network_type.value() as u32;
        data.insert("versionV", (network_type << 24) + self.version as u32);
        data.insert("signatureV", builder.create_vector(&[0u8; SIGNATURE_SIZE]).value());
        data.insert("signerV", builder.create_vector_direct(&[0u8; SIGNER_SIZE]).value());
        data.insert("deadlineV", builder.create_vector(
            &deadline.to_blockchain_timestamp().to_uint64().to_int_array()).value());

        data.insert("feeV", builder.create_vector(&max_fee.to_int_array()).value());

        return data;
    }
}

impl core::fmt::Display for AbstractTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub height: Height,
    pub index: u32,
    pub id: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_component_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agregate_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_aggregate_hash: Option<String>,
}

impl TransactionInfo {
    pub fn new(
        height: Height,
        index: u32,
        id: String,
        transaction_hash: Option<Hash>,
        merkle_component_hash: Hash,
        agregate_hash: Hash,
        aggregate_id: String,
        unique_aggregate: Hash
    ) -> Self {
        TransactionInfo {
            height,
            index,
            id,
            transaction_hash,
            merkle_component_hash: Some(merkle_component_hash),
            agregate_hash: Some(agregate_hash),
            aggregate_id: Some(aggregate_id),
            unique_aggregate_hash: Some(unique_aggregate)
        }
    }
}

impl core::fmt::Display for TransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
