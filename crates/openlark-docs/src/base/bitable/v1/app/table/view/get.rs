//! 该接口根据 view_id 检索现有视图
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetViewRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetView {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    view_id: String,
}

impl GetView {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, view_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            view_id: view_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<GetViewResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.config.base_url, self.app_token, self.table_id, self.view_id
        );
        let request = ApiRequest::get(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
