//! 该接口用于更新数据表中的一条记录
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRecordRequest {
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRecordResponse {
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

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateRecordBuilder {
    api_req: ApiRequest<UpdateRecordRequest>,
    app_token: String,
    table_id: String,
    record_id: String,
}

impl UpdateRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, record_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.record_id = record_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            builder.app_token, builder.table_id, builder.record_id
        );
        builder.api_req.body = Some(UpdateRecordRequest::default());
        builder
    }

    pub fn fields(mut self, fields: serde_json::Value) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.fields = fields;
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
