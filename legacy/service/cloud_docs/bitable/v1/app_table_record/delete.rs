use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 删除记录请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct DeleteRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符
    #[serde(skip)]
    record_id: String,
}

impl DeleteRecordRequest {
    pub fn builder() -> DeleteRecordRequestBuilder {
        DeleteRecordRequestBuilder::default()
    }

    pub fn new(
        app_token: impl ToString,
        table_id: impl ToString,
        record_id: impl ToString,
    ) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            record_id: record_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest,
}

impl DeleteRecordRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的唯一标识符
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 记录的唯一标识符
    pub fn record_id(mut self, record_id: impl ToString) -> Self {
        self.request.record_id = record_id.to_string();
        self
    }

    pub fn build(self) -> DeleteRecordRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到DeleteRecordRequestBuilder
crate::impl_executable_builder_owned!(
    DeleteRecordRequestBuilder,
    super::AppTableRecordService,
    DeleteRecordRequest,
    BaseResponse<DeleteRecordResponse>,
    delete
);

/// 删除记录响应
#[derive(Debug, Deserialize)]
pub struct DeleteRecordResponse {
    /// 是否删除成功
    pub deleted: bool,
    /// 被删除的记录 ID
    pub record_id: String,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除记录
pub async fn delete_record(
    request: DeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteRecordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = BITABLE_V1_RECORD_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{record_id}", &request.record_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_record_request_builder() {
        let request = DeleteRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("rec123456")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
    }
}
