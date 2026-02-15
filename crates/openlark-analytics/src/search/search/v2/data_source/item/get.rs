//! 查询指定数据项

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetDataSourceItemRequest {
    config: Arc<Config>,
    data_source_id: String,
    item_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataSourceItemResponse {
    pub data: Option<DataSourceItemData>,
}

impl ApiResponseTrait for GetDataSourceItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceItemData {
    pub item_id: String,
    pub data: serde_json::Value,
}

impl GetDataSourceItemRequest {
    pub fn new(
        config: Arc<Config>,
        data_source_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
            item_id: item_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetDataSourceItemResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDataSourceItemResponse> {
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items/{}",
            self.data_source_id, self.item_id
        );
        let req: ApiRequest<GetDataSourceItemResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询指定数据项", "响应数据为空")
        })
    }
}
