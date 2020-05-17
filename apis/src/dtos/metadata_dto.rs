use super::FieldDto;

/// MetadataModificationTypeEnum : The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata.
/// The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata.
#[derive(Serialize, Deserialize)]
pub enum MetadataModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

/// MetadataTypeEnum : The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata.
/// The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata.
#[derive(Serialize, Deserialize)]
pub enum MetadataTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataDto {
    #[serde(rename = "metadataType")]
    metadata_type: i32,
    #[serde(rename = "fields")]
    fields: Vec<FieldDto>,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataIds {
    #[serde(rename = "metadataIds", skip_serializing_if = "Option::is_none")]
    metadata_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataModificationDto {
    modification_type: MetadataModificationTypeEnum,
    /// The key of metadata modification.
    key: String,
    /// The value of metadata modification.
    value: String,
}
