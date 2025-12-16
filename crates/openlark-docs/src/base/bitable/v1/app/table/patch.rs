//! 更新数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/patch

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchTableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchTableResponse {
    pub name: String,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct PatchTableBuilder {
    api_req: ApiRequest<PatchTableRequest>,
    app_token: String,
    table_id: String,
}

impl PatchTableBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_table_patch".to_string();
        builder.api_req.method = "PATCH".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(PatchTableRequest::default());
        builder
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = Some(name.to_string());
        }
        self
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
