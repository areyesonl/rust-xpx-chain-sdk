use std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::models::{
    errors::ERR_EMPTY_INNER_TRANSACTION,
    multisig::Cosignature,
    network::NetworkType,
};
use crate::models::account::Account;
use crate::models::consts::{AGGREGATE_BONDED_HEADER, DEAD_LINE_SIZE, MAX_FEE_SIZE, SIGNATURE_SIZE};
use crate::models::transaction::{AbsTransaction, sign_transaction, SignedTransaction, to_aggregate_transaction_bytes};
use crate::models::transaction::schema::aggregate_transaction_schema;

use super::{
    AbstractTransaction,
    AGGREGATE_BONDED_VERSION,
    AGGREGATE_COMPLETED_VERSION, Deadline, EntityTypeEnum, Transaction, Transactions};
use super::buffer::aggregate::buffers;

/// AggregateTransaction:
/// Transaction that combines multiple transactions together.
#[derive(Debug, Serialize)]
pub struct AggregateTransaction {
    pub abs_transaction: AbstractTransaction,
    /// An array of transaction cosignatures.
    pub cosignatures: Vec<Cosignature>,
    /// The array of transactions initiated by different accounts.
    pub inner_transactions: Transactions,
}

impl AggregateTransaction {
    pub fn new_complete(deadline: Deadline, inner_txs: Vec<Box<dyn Transaction>>,
                        network_type: NetworkType) -> crate::Result<AggregateTransaction>
    {
        ensure!(
            inner_txs.len() > 0,
            ERR_EMPTY_INNER_TRANSACTION
         );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            AGGREGATE_COMPLETED_VERSION,
            EntityTypeEnum::AggregateComplete,
            network_type
        );

        Ok(Self { abs_transaction: abs_tx, cosignatures: vec![], inner_transactions: inner_txs })
    }

    pub fn new_bonded(deadline: Deadline, inner_txs: Vec<Box<dyn Transaction>>,
                      network_type: NetworkType) -> crate::Result<Self>
    {
        ensure!(
            inner_txs.len() > 0,
            ERR_EMPTY_INNER_TRANSACTION
         );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            AGGREGATE_BONDED_VERSION,
            EntityTypeEnum::AggregateBonded,
            network_type
        );

        Ok(Self { abs_transaction: abs_tx, cosignatures: vec![], inner_transactions: inner_txs })
    }
}

impl fmt::Display for AggregateTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl AbsTransaction for AggregateTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for AggregateTransaction {
    fn size(&self) -> usize {
        let mut size_of_inner_transactions = 0;
        self.inner_transactions.iter().for_each(|itx|
            size_of_inner_transactions += itx.size() - SIGNATURE_SIZE - MAX_FEE_SIZE - DEAD_LINE_SIZE
        );
        AGGREGATE_BONDED_HEADER + size_of_inner_transactions
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mut txsb: Vec<u8> = Vec::new();
        for tx in &self.inner_transactions {
            let mut tx_byte = to_aggregate_transaction_bytes(tx).unwrap();
            txsb.append(&mut tx_byte)
        }

        let tx_vec = _builder.create_vector(&txsb);

        let abs_vector = &self.abs_transaction.generate_vector(&mut _builder);

        let mut txn_builder =
            buffers::AggregateTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));
        txn_builder.add_transactions_size(txsb.len() as u32);
        txn_builder.add_transactions(tx_vec);

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        aggregate_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}