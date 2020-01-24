#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataBodyDto {
    /// The address in hexadecimal.
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl AddressMetadataBodyDto {
    pub fn new(metadata_id: String, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> AddressMetadataBodyDto {
        AddressMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
}

impl AddressMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: String) -> AddressMetadataDto {
        AddressMetadataDto {
            metadata_type,
            fields,
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataDtoAllOf {
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
}

impl AddressMetadataDtoAllOf {
    pub fn new(metadata_id: String) -> AddressMetadataDtoAllOf {
        AddressMetadataDtoAllOf {
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: crate::models::account::AddressMetadataDto,
}

impl AddressMetadataInfoDto {
    pub fn new(metadata: crate::models::account::AddressMetadataDto) -> AddressMetadataInfoDto {
        AddressMetadataInfoDto {
            metadata,
        }
    }
}

/// AddressMetadataTransactionDto : Transaction that addes metadata to account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The address in hexadecimal.
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl AddressMetadataTransactionDto {
    /// Transaction that addes metadata to account.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: String, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> AddressMetadataTransactionDto {
        AddressMetadataTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedAddressMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The address in hexadecimal.
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl EmbeddedAddressMetadataTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: String, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> EmbeddedAddressMetadataTransactionDto {
        EmbeddedAddressMetadataTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}
