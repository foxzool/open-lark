//! 创建数据知识
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSETS};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 创建数据知识请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateDataAssetBody {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// 创建数据知识请求
pub struct CreateDataAssetRequest {
    config: Config,
    app_id: String,
}

impl CreateDataAssetRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    pub async fn execute(self, body: CreateDataAssetBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateDataAssetBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_DATA_ASSETS.replace("{app_id}", &self.app_id);
        let req: ApiRequest<CreateDataAssetBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建数据知识")
    }
}
