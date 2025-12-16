//! 该接口用于根据 record_id 的值检索现有记录
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetRecordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field_as_array: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_formula_ref: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_shared_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetRecordResponse {
    pub record: AppTableRecord,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppTableRecord {
    pub record_id: String,
    pub created_by: Option<User>,
    pub created_time: Option<i64>,
    pub last_modified_by: Option<User>,
    pub last_modified_time: Option<i64>,
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub en_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetRecordBuilder {
    api_req: ApiRequest<GetRecordRequest>,
    app_token: String,
    table_id: String,
    record_id: String,
}

impl GetRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, record_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.record_id = record_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            builder.app_token, builder.table_id, builder.record_id
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
