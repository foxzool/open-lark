//! 获取表单的所有元数据项
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetFormRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetFormResponse {
    pub form: Form,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Form {
    pub name: String,
    pub description: String,
    pub shared: bool,
    pub shared_url: String,
    pub shared_limit: String, // "off", "tenant_editable", "anyone_editable"
    pub submit_limit_once: bool,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetForm {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetForm {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, form_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            form_id: form_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<GetFormResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            self.config.base_url, self.app_token, self.table_id, self.form_id
        );
        let request = ApiRequest::get(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
