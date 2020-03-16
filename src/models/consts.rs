//pub const ACCOUNT_LINK_TRANSACTION_SIZE: usize = TRANSACTION_HEADER_SIZE + KEY_SIZE + LINK_ACTION_SIZE;
//pub const ACCOUNT_PROPERTIES_ADDRESS_MODIFICATION_SIZE: usize = PROPERTY_MODIFICATION_TYPE_SIZE + ADDRESS_DECODE_SIZE;
//pub const ACCOUNT_PROPERTIES_ENTITY_MODIFICATION_SIZE: usize = PROPERTY_MODIFICATION_TYPE_SIZE + TYPE_SIZE;
//pub const ACCOUNT_PROPERTIES_MOSAIC_MODIFICATION_SIZE: usize = PROPERTY_MODIFICATION_TYPE_SIZE + MOSAIC_ID_SIZE;
//pub const ACCOUNT_PROPERTY_ADDRESS_HEADER: usize = TRANSACTION_HEADER_SIZE + PROPERTY_TYPE_SIZE;
//pub const ACCOUNT_PROPERTY_ENTITY_TYPE_HEADER: usize = TRANSACTION_HEADER_SIZE + PROPERTY_TYPE_SIZE;
//pub const ACCOUNT_PROPERTY_MOSAIC_HEADER: usize = TRANSACTION_HEADER_SIZE + PROPERTY_TYPE_SIZE;
pub const ADDRESS_DECODE_SIZE: usize = 25;
pub const ADDRESS_ENCODE_SIZE: usize = 40;
pub const ADDRESS_SIZE: usize = 25;
//pub const ADD_EXCHANGE_OFFER_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + OFFERS_COUNT_SIZE;
//pub const ADD_EXCHANGE_OFFER_SIZE: usize = MOSAIC_ID_SIZE + DURATION_SIZE + 2 * AMOUNT_SIZE + OFFER_TYPE_SIZE;
pub const AGGREGATE_BONDED_HEADER: usize = TRANSACTION_HEADER_SIZE + SIZE_SIZE;
//pub const ALIAS_ACTION_SIZE: usize = 1;
//pub const ALIAS_TRANSACTION_HEADER: usize = TRANSACTION_HEADER_SIZE + NAMESPACE_SIZE + ALIAS_ACTION_SIZE;
pub const AMOUNT_SIZE: usize = 8;
pub const BASE_INT64SIZE: usize = 8;
pub const DEAD_LINE_SIZE: usize = 8;
pub const DURATION_SIZE: usize = 8;
//pub const EXCHANGE_OFFER_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + OFFERS_COUNT_SIZE;
//pub const EXCHANGE_OFFER_SIZE: usize = DURATION_SIZE + 2 * AMOUNT_SIZE + OFFER_TYPE_SIZE + KEY_SIZE;
pub const HALF_OF_SIGNATURE: usize = SIGNATURE_SIZE / 2;
pub const HASH256: usize = 32;
//pub const HASH_TYPE_SIZE: usize = 1;
pub const KEY_SIZE: usize = 32;
//pub const LINK_ACTION_SIZE: usize = 1;
pub const LOCK_SIZE: usize = TRANSACTION_HEADER_SIZE + MOSAIC_ID_SIZE + AMOUNT_SIZE + DURATION_SIZE + HASH256;
pub const MAX_FEE_SIZE: usize = 8;
pub const MESSAGE_SIZE_SIZE: usize = 2;
//pub const METADATA_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + METADATA_TYPE_SIZE;
//pub const METADATA_TYPE_SIZE: usize = 1;
pub const MIN_APPROVAL_SIZE: usize = 1;
pub const MIN_REMOVAL_SIZE: usize = 1;
pub const MODIFICATIONS_SIZE_SIZE: usize = 1;
//pub const MODIFY_CONTRACT_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + DURATION_SIZE + HASH256 + 3 * MODIFICATIONS_SIZE_SIZE;
pub const MODIFY_MULTISIG_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + MIN_APPROVAL_SIZE + MIN_REMOVAL_SIZE + MODIFICATIONS_SIZE_SIZE;
pub const MOSAICS_SIZE_SIZE: usize = 1;
pub const MOSAIC_DEFINITION_TRANSACTION_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + MOSAIC_NONCE_SIZE + MOSAIC_ID_SIZE + MOSAIC_PROPERTIES_HEADER_SIZE;
pub const MOSAIC_ID_SIZE: usize = 8;
pub const MOSAIC_NONCE_SIZE: usize = 4;
pub const MOSAIC_PROPERTY_ID_SIZE: usize = 1;
pub const MOSAIC_OPTIONAL_PROPERTY_SIZE: usize = MOSAIC_PROPERTY_ID_SIZE + BASE_INT64SIZE;
pub const MOSAIC_PROPERTIES_HEADER_SIZE: usize = 3;
//pub const MOSAIC_PROPERTY_SIZE: usize = 4;
pub const MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE: usize = TRANSACTION_HEADER_SIZE + MOSAIC_ID_SIZE + AMOUNT_SIZE + MOSAIC_SUPPLY_DIRECTION_SIZE;
pub const MOSAIC_SUPPLY_DIRECTION_SIZE: usize = 1;
pub const NAMESPACE_NAME_SIZE_SIZE: usize = 1;
pub const NAMESPACE_SIZE: usize = 8;
pub const NAMESPACE_TYPE_SIZE: usize = 1;
//pub const NUM_CHECKSUM_BYTES: usize = 4;
//pub const OFFERS_COUNT_SIZE: usize = 1;
//pub const OFFER_TYPE_SIZE: usize = 1;
//pub const PRIVATE_KEY_SIZE: usize = 64;
//pub const PROOF_SIZE_SIZE: usize = 2;
//pub const PROPERTY_MODIFICATION_TYPE_SIZE: usize = 1;
//pub const PROPERTY_TYPE_SIZE: usize = 2;
//pub const PUBLIC_KEY_SIZE: usize = 64;
pub const REGISTER_NAMESPACE_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + NAMESPACE_TYPE_SIZE + DURATION_SIZE + NAMESPACE_SIZE + NAMESPACE_NAME_SIZE_SIZE;
//pub const REMOVE_EXCHANGE_OFFER_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + OFFERS_COUNT_SIZE;
//pub const REMOVE_EXCHANGE_OFFER_SIZE: usize = OFFER_TYPE_SIZE + MOSAIC_ID_SIZE;
//pub const SECRET_LOCK_SIZE: usize = TRANSACTION_HEADER_SIZE + MOSAIC_ID_SIZE + AMOUNT_SIZE + DURATION_SIZE + HASH_TYPE_SIZE + HASH256 + ADDRESS_DECODE_SIZE;
//pub const SECRET_PROOF_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + HASH_TYPE_SIZE + HASH256 + PROOF_SIZE_SIZE;
pub const SIGNATURE_SIZE: usize = 64;
pub const SIGNER_SIZE: usize = KEY_SIZE;
pub const SIZE_SIZE: usize = 4;
pub const TRANSACTION_HEADER_SIZE: usize = SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE + VERSION_SIZE + TYPE_SIZE + MAX_FEE_SIZE + DEAD_LINE_SIZE;
pub const TRANSFER_HEADER_SIZE: usize = TRANSACTION_HEADER_SIZE + ADDRESS_DECODE_SIZE + MOSAICS_SIZE_SIZE + MESSAGE_SIZE_SIZE;
pub const TYPE_SIZE: usize = 2;
pub const VERSION_SIZE: usize = 4;