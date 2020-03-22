use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::models::account::{Account, PublicAccount};
use crate::models::consts::REGISTER_NAMESPACE_HEADER_SIZE;
use crate::models::errors;
use crate::models::id_model::Id;
use crate::models::namespace::{generate_namespace_id, NamespaceId, NamespaceType};
use crate::models::network::NetworkType;
use crate::models::transaction::AbsTransaction;
use crate::models::Uint64;

use super::{
    AbstractTransaction,
    buffer::register_namespace::buffers,
    Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    REGISTER_NAMESPACE_VERSION,
    schema::register_namespace_transaction_schema,
    SignedTransaction,
    Transaction
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransaction {
    pub abs_transaction: AbstractTransaction,
    pub namespace_type: NamespaceType,
    pub namespace_id: NamespaceId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Uint64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NamespaceId>
}

impl RegisterNamespaceTransaction {
    pub fn create_root(
        deadline: Deadline,
        namespace_name: &str,
        duration: Uint64,
        network_type: NetworkType,
    ) -> crate::Result<RegisterNamespaceTransaction> {
        ensure!(
            namespace_name.len() != 0 && namespace_name.len() <= 16 ,
            errors::ERR_INVALID_NAMESPACE_NAME
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type);

        let namespace_id = NamespaceId::from_name(namespace_name)?;

        Ok(RegisterNamespaceTransaction {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Root,
            namespace_id,
            name: namespace_name.parse().unwrap(),
            duration: Some(duration),
            parent_id: None
        })
    }

    pub fn create_sub(
        deadline: Deadline,
        namespace_name: &'static str,
        parent_id: NamespaceId,
        network_type: NetworkType,
    ) -> crate::Result<Self> {
        ensure!(
            namespace_name.len() != 0 && namespace_name.len() <= 64 ,
            errors::ERR_INVALID_NAMESPACE_NAME
        );

        ensure!(
            parent_id.to_u64() != 0,
            errors::ERR_NULL_NAMESPACE_ID
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type);

        let namespace_id = generate_namespace_id(namespace_name, parent_id)?;

        Ok(Self {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Sub,
            namespace_id,
            name: namespace_name.parse().unwrap(),
            duration: None,
            parent_id: Some(parent_id)
        })
    }
}

impl AbsTransaction for RegisterNamespaceTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for RegisterNamespaceTransaction {
    fn size(&self) -> usize {
        REGISTER_NAMESPACE_HEADER_SIZE + self.name.len()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let namespace_id_vec = builder.create_vector(&self.namespace_id.to_u32_array());

        let mut d_vec = fb::WIPOffset::new(0);
        if self.namespace_type == NamespaceType::Root {
            d_vec = builder.create_vector(&self.duration.unwrap().to_int_array());
        } else {
            d_vec = builder.create_vector(&self.parent_id.unwrap().to_u32_array());
        }

        let name_vec = builder.create_string(self.name.as_ref());

        let abs_vector = &self.abs_transaction.generate_vector(&mut builder);

        let mut txn_builder =
            buffers::RegisterNamespaceTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));

        txn_builder.add_namespace_type(self.namespace_type.clone() as u8);
        txn_builder.add_duration_parent_id(d_vec);
        txn_builder.add_namespace_id(namespace_id_vec);
        txn_builder.add_namespace_name_size(self.name.len() as u8);
        txn_builder.add_namespace_name(name_vec);

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        register_namespace_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for RegisterNamespaceTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
