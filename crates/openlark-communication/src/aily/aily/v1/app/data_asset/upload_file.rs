//! 上传文件用于数据知识管理
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_UPLOAD_FILE};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 上传文件用于数据知识管理请求
pub struct UploadFileRequest {
    config: Config,
    app_id: String,
}

impl UploadFileRequest {
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

    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_UPLOAD_FILE.replace("{app_id}", &self.app_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "上传文件用于数据知识管理")
    }
}
