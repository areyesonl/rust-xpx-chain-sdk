use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;


use super::request as __internal_request;
use crate::apis::sirius_client::ApiClient;
use crate::models::mosaic::{MosaicInfo, MosaicInfoDto, MosaicIds, Mosaic, MosaicId};
use crate::models::Id;

#[derive(Debug, Clone)]
pub struct MosaicRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> MosaicRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> MosaicRoutesApiClient<C> {
        let clone = client.clone();

        MosaicRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> MosaicRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_mosaic_info(self, mosaic_id: MosaicId) -> super::Result<MosaicInfo> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/mosaic/{mosaicId}".to_string()
        );

        req = req.with_path_param("mosaicId".to_string(), mosaic_id.to_string());

        let dto: super::Result<MosaicInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct()?)
    }

    pub async fn get_mosaics_info(self, mosaics_id: Vec<MosaicId>) -> super::Result<Vec<MosaicInfo>> {
        let mosaics_ids = MosaicIds::from(mosaics_id);
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/mosaic".to_string()
        );

        req = req.with_body_param(mosaics_ids);

        let dto: Vec<MosaicInfoDto> = req.execute(self.client).await?;

        let mut mosaics_info: Vec<MosaicInfo> = Vec::with_capacity(dto.len());
        for i in dto {
            let mosaic_info = i;
            mosaics_info.push(mosaic_info.to_struct()?);
        }

        Ok(mosaics_info)
    }

    pub async fn get_mosaics_names(self, mosaic_ids: MosaicIds) -> super::Result<()> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/mosaic/names".to_string()
        );

        req = req.with_body_param(mosaic_ids);

        req.execute(self.client).await
    }
}