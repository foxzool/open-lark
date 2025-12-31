/// Bitable 更新记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/update
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Record;

/// 更新记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UpdateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
    user_id_type: Option<String>,
    ignore_consistency_check: Option<bool>,
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            ignore_consistency_check: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 是否忽略一致性读写检查
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateRecordResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.record_id.trim().is_empty() {
            return Err(validation_error("record_id", "record_id 不能为空"));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RecordUpdate(
            self.app_token.clone(),
            self.table_id.clone(),
            self.record_id,
        );

        let mut api_request: ApiRequest<UpdateRecordResponse> = ApiRequest::put(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&UpdateRecordRequestBody {
            fields: self.fields,
        })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt(
            "ignore_consistency_check",
            self.ignore_consistency_check.map(|v| v.to_string()),
        );

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 更新记录 Builder
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateRecordRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.request = self.request.record_id(record_id);
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.request = self
            .request
            .ignore_consistency_check(ignore_consistency_check);
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.request = self.request.fields(fields);
        self
    }

    pub fn build(self) -> UpdateRecordRequest {
        self.request
    }
}

#[derive(Serialize)]
struct UpdateRecordRequestBody {
    fields: Value,
}

/// 更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRecordResponse {
    /// 更新记录的内容
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
