//! 在数据表中新增一个视图
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateViewRequest {
    pub view_name: String,
    pub view_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
}

impl ApiResponseTrait for CreateViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateView {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: CreateViewRequest,
}

impl CreateView {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: CreateViewRequest::default(),
        }
    }

    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.req.view_name = view_name.into();
        self
    }

    pub fn view_type(mut self, view_type: impl Into<String>) -> Self {
        self.req.view_type = view_type.into();
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateViewResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
