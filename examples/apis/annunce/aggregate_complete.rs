#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, Transaction, TransferTransaction,
};

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
const PRIVATE_KEY: &str = "6D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    // Deadline default 1 hour
    let deadline = Deadline::default();
    //let deadline = Deadline::new(1, 30, 0);

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let transfer_transaction_a = TransferTransaction::new(
        deadline,
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer A From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_a = match transfer_transaction_a {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_a.to_aggregate(account.public_account_to_owned());

    let transfer_transaction_b = TransferTransaction::new(
        deadline,
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer B From ProximaX Rust SDK"),
        network_type,
    );

    let mut transfer_b = match transfer_transaction_b {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_b.to_aggregate(account.public_account_to_owned());

    let aggregate_complete = AggregateTransaction::new_complete(
        deadline,
        vec![Box::new(transfer_a), Box::new(transfer_b)],
        network_type,
    );

    let sig_transaction = account.sign(aggregate_complete.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
