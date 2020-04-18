use sha3::{Digest, Sha3_256};

use utils::array_u8_to_u64;

use crate::models::{account::PublicAccount, namespace::NAMESPACE_BIT, Uint64};

use super::MosaicNonce;

pub(crate) static XPX_DIVISIBILITY: u64 = 1000000;

pub(crate) static XPX_MAX_VALUE: u64 = XPX_MAX_RELATIVE_VALUE * XPX_DIVISIBILITY;

pub(crate) static XPX_MAX_RELATIVE_VALUE: u64 = 9000000000;

pub(crate) static PRX_XPX_U64: u64 = 13833723942089965046;

pub(crate) fn generate_mosaic_id(nonce: MosaicNonce, owner_public_id: PublicAccount) -> Uint64 {
    let mut hash = Sha3_256::default();

    let nonce_bytes = nonce.to_array();

    hash.input(nonce_bytes);

    let owner_bytes: [u8; 32] = owner_public_id.to_bytes();

    hash.input(owner_bytes);

    let hash_to_array = hash.result();

    Uint64(array_u8_to_u64(&mut hash_to_array.as_slice()) ^ NAMESPACE_BIT)
}