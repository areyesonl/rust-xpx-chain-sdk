use ::core::fmt;

use serde::{Serialize, Serializer};

use crate::models::id_model::Id;
use crate::models::{errors, Uint64};
use utils::has_bits;

use super::{generate_namespace_path, NAMESPACE_BIT};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Copy)]
pub struct NamespaceId(pub(crate) Uint64);

impl NamespaceId {
    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn new(id: u64) -> NamespaceId {
        assert!(
            id != 0 && has_bits(id, NAMESPACE_BIT),
            errors::ERR_WRONG_BIT_NAMESPACE_ID
        );

        NamespaceId(Uint64::new(id))
    }

    /// Creates a new `NamespaceId` from a hex string.
    pub fn from_name(string_name: &str) -> crate::Result<NamespaceId> {
        ensure!(!string_name.is_empty(), errors::ERR_EMPTY_NAMESPACE_NAME);

        let list = generate_namespace_path(string_name)?;

        ensure!(!list.is_empty(), errors::ERR_INVALID_NAMESPACE_NAME);

        Ok(list[list.len() - 1])
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: u32, higher: u32) -> NamespaceId {
        NamespaceId(Uint64::from_ints(lower, higher))
    }
}

impl Id for NamespaceId {
    fn to_uint64(&self) -> Uint64 {
        self.0
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Serialize for NamespaceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl From<Uint64> for NamespaceId {
    fn from(e: Uint64) -> Self {
        return NamespaceId(e);
    }
}

impl From<&str> for NamespaceId {
    fn from(hex: &str) -> Self {
        return NamespaceId::from(Uint64::from_hex(hex).unwrap());
    }
}