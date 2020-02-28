use std::collections::HashMap;

use fb::FlatBufferBuilder;

use crate::models::account::PublicAccount;
use crate::models::consts::{SIGNATURE_SIZE, SIGNER_SIZE};
use crate::models::network::network_internal::extract_network_type;
use crate::models::network::NetworkType;
use crate::models::transaction::{deadline::Deadline, EntityTypeEnum};
use crate::models::transaction::EntityVersion;
use crate::models::Uint64;

use super::buffer::transfer::buffers;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) transaction_info: Option<TransactionInfo>,

    pub(crate) network_type: NetworkType,

    /// The signature was generated by the signer and can be used to validate tha the entity
    /// data was not modified by a node.
    pub(crate) signature: String,

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
    pub(crate) max_fee: Uint64,

    /// The 'Deadline' for the transaction to be included in a block before it expires.
    pub(crate) deadline: Deadline,
}

impl AbstractTransaction {
    pub fn new(tx_info: Option<TransactionInfo>,
               network_type: NetworkType,
               signature: String,
               signer: PublicAccount,
               version: EntityVersion,
               transaction_type: EntityTypeEnum,
               max_fee: Uint64,
               deadline: Deadline,
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

    pub fn new_from_type(deadline: Deadline,
                         version: EntityVersion,
                         transaction_type: EntityTypeEnum, network_type: NetworkType) -> Self {
        AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: "".to_string(),
            signer: Default::default(),
            version,
            transaction_type,
            max_fee: Default::default(),
            deadline
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
        self.signer = signer
    }

    pub fn generate_vector(&self, builder: &mut FlatBufferBuilder) -> HashMap<&str, fb::UOffsetT> {
        let mut data: HashMap<&str, fb::UOffsetT> = HashMap::new();

        let network_type: fb::UOffsetT = self.network_type.0 as u32;
        data.insert("versionV", (network_type << 24) + self.version as u32);
        data.insert("signatureV", builder.create_vector(&[0u8; SIGNATURE_SIZE]).value());
        data.insert("signerV", builder.create_vector_direct(&[0u8; SIGNER_SIZE]).value());
        data.insert("deadlineV", builder.create_vector(
            &self.deadline.to_blockchain_timestamp().to_uint64().to_int_array()).value());

        data.insert("feeV", builder.create_vector(&self.max_fee.to_int_array()).value());

        return data;
    }
    pub fn build_vector(&self, builder: &mut FlatBufferBuilder,vector: &HashMap<&str, fb::UOffsetT>,
    ) {
        let mut transaction = buffers::TransferTransactionBufferBuilder::new(builder);
        transaction.add_signature(fb::WIPOffset::new(*vector.get("signatureV").unwrap()));
        transaction.add_signer(fb::WIPOffset::new(*vector.get("signerV").unwrap()));
        transaction.add_version(*vector.get("versionV").unwrap());
        transaction.add_type_(self.transaction_type.get_value());
        transaction.add_max_fee(fb::WIPOffset::new(*vector.get("feeV").unwrap()));
        transaction.add_deadline(fb::WIPOffset::new(*vector.get("deadlineV").unwrap()));
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
    pub height: Uint64,
    pub index: u32,
    pub id: String,
    pub transaction_hash: String,
    pub merkle_component_hash: String,
    pub agregate_hash: String,
    pub aggregate_id: String,
}

impl TransactionInfo {
    pub fn new(
        height: Uint64,
        index: u32,
        id: String,
        transaction_hash: String,
        merkle_component_hash: String,
        agregate_hash: String,
        aggregate_id: String,
    ) -> Self {
        TransactionInfo {
            height,
            index,
            id,
            transaction_hash,
            merkle_component_hash,
            agregate_hash,
            aggregate_id,
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
