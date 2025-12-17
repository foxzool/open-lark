//! 删除数据表中的视图
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteViewRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteViewResponse {}

impl ApiResponseTrait for DeleteViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct DeleteView {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    view_id: String,
}

impl DeleteView {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, view_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            view_id: view_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<DeleteViewResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.config.base_url, self.app_token, self.table_id, self.view_id
        );
        let request = ApiRequest::delete(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
