//! 获取数据知识
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/get

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSET};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 获取数据知识请求
pub struct GetDataAssetRequest {
    config: Config,
    app_id: String,
    data_asset_id: String,
}

impl GetDataAssetRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            data_asset_id: String::new(),
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    pub fn data_asset_id(mut self, data_asset_id: impl Into<String>) -> Self {
        self.data_asset_id = data_asset_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.data_asset_id, "data_asset_id 不能为空");

        let url = AILY_V1_DATA_ASSET
            .replace("{app_id}", &self.app_id)
            .replace("{data_asset_id}", &self.data_asset_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取数据知识")
    }
}
