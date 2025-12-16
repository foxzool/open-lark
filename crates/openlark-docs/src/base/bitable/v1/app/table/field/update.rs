//! 该接口用于在数据表中更新一个字段
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateFieldRequest {
    pub field_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateFieldResponse {
    pub field: Field,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Field {
    pub field_id: String,
    pub field_name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateFieldBuilder {
    api_req: ApiRequest<UpdateFieldRequest>,
    app_token: String,
    table_id: String,
    field_id: String,
}

impl UpdateFieldBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, field_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_field_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.field_id = field_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
            builder.app_token, builder.table_id, builder.field_id
        );
        builder.api_req.body = Some(UpdateFieldRequest::default());
        builder
    }

    pub fn field_name(mut self, field_name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.field_name = field_name.to_string();
        }
        self
    }

    pub fn property(mut self, property: serde_json::Value) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.property = Some(property);
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
