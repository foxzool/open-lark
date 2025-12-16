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

#[derive(Debug, Default)]
pub struct GetFormBuilder {
    api_req: ApiRequest<GetFormRequest>,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, form_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_form_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.form_id = form_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            builder.app_token, builder.table_id, builder.form_id
        );
        builder.api_req.body = None;
        builder
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
