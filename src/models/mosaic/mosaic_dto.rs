use crate::{
    models::{
        field_dto::FieldDto,
        metadata_dto::{MetadataModificationDto, MetadataTypeEnum},
        Uint64,
        uint_64::Uint64Dto,
    },
    Result,
};

use super::{
    internally::has_bits,
    Mosaic,
    MosaicId,
    MosaicInfo,
    MosaicNames,
    MosaicProperties, SUPPLY_MUTABLE,
    TRANSFERABLE,
};
use crate::models::transaction::{TransactionMetaDto, TransactionDto, Transaction, AbstractTransactionDto, MosaicDefinitionTransaction, MosaicSupplyChangeTransaction};
use failure::_core::any::Any;
use crate::models::mosaic::{MosaicNonce, MosaicSupplyType};
use crate::models::mosaic::internally::mosaic_properties;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicDto {
    #[serde(rename = "id")]
    id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

impl MosaicDto {
    pub fn to_struct(&self) -> Mosaic {
        let mosaic_id = MosaicId::from(self.id.to_struct());
        let amount = self.amount.to_struct();
        Mosaic::new(mosaic_id, amount)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicInfoDto {
    #[serde(rename = "meta")]
    meta: MosaicMetaDto,
    #[serde(rename = "mosaic")]
    mosaic: MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn to_struct(&self) -> Result<MosaicInfo> {
        ensure!(
            self.mosaic.properties.len() > 0,
            "mosaic Properties is not valid."
         );

        let mosaic_id = MosaicId::from(self.mosaic.mosaic_id.to_struct());

        let properties = mosaic_properties(&self.mosaic.properties)?;

        Ok(MosaicInfo::new(
            mosaic_id,
            self.mosaic.supply.to_struct(),
            self.mosaic.height.to_struct(),
            (&self.mosaic.owner).parse()?,
            self.mosaic.revision,
            properties,
        ))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MosaicMetaDto {
    #[serde(rename = "id")]
    id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataBodyDto {
    metadata_id: Uint64Dto,
    metadata_type: u16,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicMetadataDto {
    pub metadata_type: i32,
    pub fields: Vec<FieldDto>,
    pub metadata_id: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    metadata_id: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: MosaicMetadataDto,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicDefinitionTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: MosaicDefinitionTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicDefinitionTransactionInfoDto {
    fn version(&self) -> i32 {
        self.transaction.version
    }

    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct()?;

        let properties = mosaic_properties(&dto.properties)?;

        Ok(Box::new(MosaicDefinitionTransaction{
            abs_transaction: abs,
            properties,
            mosaic_nonce: MosaicNonce::from(dto.mosaic_nonce as u32),
            mosaic_id: MosaicId::from(dto.mosaic_id.to_struct())
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionDto {
    mosaic_id: Uint64Dto,
    supply: Uint64Dto,
    height: Uint64Dto,
    owner: String,
    revision: usize,
    properties: Vec<MosaicPropertyDto>,
}

/// MosaicDefinitionTransactionDto : Transaction that creates a new mosaic.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicDefinitionTransactionDto {
    signature: String,
    signer: String,
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// Random nonce used to generate the mosaic id.
    mosaic_nonce: i32,
    mosaic_id: Uint64Dto,
    properties: Vec<MosaicPropertyDto>,
}

/// MosaicMetadataTransactionDto : Transaction that addes metadata to mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicNamesDto {
    mosaic_id: Uint64Dto,
    names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn to_struct(&self) -> MosaicNames {
        MosaicNames::new(
            MosaicId::from(self.mosaic_id.to_struct()),
            (self.names).to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicPropertyDto {
    pub(crate) id: u8,
    pub(crate) value: Uint64Dto,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicSupplyChangeTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: MosaicSupplyChangeTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicSupplyChangeTransactionInfoDto {
    fn version(&self) -> i32 {
        self.transaction.version
    }

    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct()?;

        Ok(Box::new(MosaicSupplyChangeTransaction{
            abs_transaction: abs,
            supply_type: MosaicSupplyType::from(dto.direction),
            asset_id: Box::new(MosaicId::from(dto.mosaic_id.to_struct())),
            delta: dto.delta.to_struct()
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// MosaicSupplyChangeTransactionDto : Transaction to increase or decrease a mosaic’s supply.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicSupplyChangeTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmbeddedMosaicMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}