use super::{FieldDto, MetadataModificationDto, Uint64Dto};

#[derive(Serialize, Deserialize)]
struct AddressMetadataBodyDto {
    /// The address in hexadecimal.
    #[serde(rename = "metadataId")]
    metadata_id: String,
    #[serde(rename = "metadataType")]
    metadata_type: u8,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
struct AddressMetadataDto {
    #[serde(rename = "metadataType")]
    metadata_type: u32,
    #[serde(rename = "fields")]
    fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    metadata_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AddressMetadataDtoAllOf {
    #[serde(rename = "metadataId")]
    metadata_id: String,
}

#[derive(Serialize, Deserialize)]
struct AddressMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: AddressMetadataDto,
}

/// AddressMetadataTransactionDto : Transaction that addes metadata to account.
#[derive(Serialize, Deserialize)]
struct AddressMetadataTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// The address in hexadecimal.
    metadata_id: String,
    metadata_type: u8,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
struct EmbeddedAddressMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    deadline: Uint64Dto,
    /// The address in hexadecimal.
    metadata_id: String,
    metadata_type: u8,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}