/// Bitable 删除记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/delete
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeleteRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
}

impl DeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
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

    pub async fn execute(self) -> SDKResult<DeleteRecordResponse> {
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
        let api_endpoint =
            BitableApiV1::RecordDelete(self.app_token, self.table_id, self.record_id);

        let api_request: ApiRequest<DeleteRecordResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 删除记录 Builder
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest,
}

impl DeleteRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteRecordRequest::new(config),
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

    pub fn build(self) -> DeleteRecordRequest {
        self.request
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteRecordResponse {
    /// 是否成功删除
    pub deleted: bool,
    /// 删除的记录 ID
    pub record_id: String,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
