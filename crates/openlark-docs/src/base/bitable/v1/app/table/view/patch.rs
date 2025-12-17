//! 该接口用于增量修改视图信息
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/patch

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchViewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct PatchView {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    view_id: String,
    req: PatchViewRequest,
}

impl PatchView {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, view_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            view_id: view_id.into(),
            req: PatchViewRequest::default(),
        }
    }

    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.req.view_name = Some(view_name.into());
        self
    }

    pub fn property(mut self, property: serde_json::Value) -> Self {
        self.req.property = Some(property);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<PatchViewResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.config.base_url, self.app_token, self.table_id, self.view_id
        );
        let request = ApiRequest::patch(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
