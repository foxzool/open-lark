//! Bitable 删除记录
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除记录请求
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
        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        validate_required_field("app_token", Some(&self.app_token), "app_token 不能为空")?;
        validate_required_field("table_id", Some(&self.table_id), "table_id 不能为空")?;
        validate_required_field("record_id", Some(&self.record_id), "record_id 不能为空")?;

        let api_endpoint =
            BitableApiV1::RecordDelete(self.app_token, self.table_id, self.record_id);
        let request = ApiRequest::<DeleteRecordResponse>::delete(&api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "删除记录")
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
///
/// 表示记录是否成功删除以及被删除记录的ID。
///
/// # 示例
/// ```json
/// {
///   "deleted": true,
///   "record_id": "recxxxxxxxxxxxx"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteRecordResponse {
    /// 是否成功删除
    ///
    /// - `true`: 删除成功
    /// - `false`: 删除失败（通常不会出现，失败会抛出异常）
    pub deleted: bool,
    /// 删除的记录 ID
    ///
    /// 被删除记录的唯一标识符
    pub record_id: String,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
