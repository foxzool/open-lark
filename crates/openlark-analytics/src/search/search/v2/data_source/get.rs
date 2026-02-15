//! 获取数据源

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
pub struct GetDataSourceRequest {
    config: Arc<Config>,
    data_source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataSourceResponse {
    pub data: Option<DataSourceData>,
}

impl ApiResponseTrait for GetDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceData {
    pub data_source_id: String,
    pub name: String,
    pub description: String,
}

impl GetDataSourceRequest {
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDataSourceResponse> {
        let path = format!("/open-apis/search/v2/data_sources/{}", self.data_source_id);
        let req: ApiRequest<GetDataSourceResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取数据源", "响应数据为空")
        })
    }
}
