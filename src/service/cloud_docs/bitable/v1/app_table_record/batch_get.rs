use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::bitable::v1::Record,
};

/// 批量获取记录请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct BatchGetRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 记录 ID 列表
    record_ids: Vec<String>,
    /// 控制是否返回自动计算的字段
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic: Option<bool>,
    /// 控制是否返回记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    with_shared_url: Option<bool>,
}

impl BatchGetRecordRequest {
    pub fn builder() -> BatchGetRecordRequestBuilder {
        BatchGetRecordRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BatchGetRecordRequestBuilder {
    request: BatchGetRecordRequest,
}

impl BatchGetRecordRequestBuilder {
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

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 记录 ID 列表
    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.request.record_ids = record_ids;
        self
    }

    /// 添加单个记录 ID
    pub fn add_record_id(mut self, record_id: impl ToString) -> Self {
        self.request.record_ids.push(record_id.to_string());
        self
    }

    /// 控制是否返回自动计算的字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    /// 控制是否返回记录权限
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.request.with_shared_url = Some(with_shared_url);
        self
    }

    pub fn build(mut self) -> BatchGetRecordRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        if let Some(automatic) = &self.request.automatic {
            self.request
                .api_request
                .query_params
                .insert("automatic", automatic.to_string());
        }
        if let Some(with_shared_url) = &self.request.with_shared_url {
            self.request
                .api_request
                .query_params
                .insert("with_shared_url", with_shared_url.to_string());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchGetRecordRequestBuilder
crate::impl_executable_builder_owned!(
    BatchGetRecordRequestBuilder,
    super::AppTableRecordService,
    BatchGetRecordRequest,
    BaseResponse<BatchGetRecordResponse>,
    batch_get
);

/// 批量获取记录响应
#[derive(Debug, Deserialize)]
pub struct BatchGetRecordResponse {
    /// 记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchGetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取记录
pub async fn batch_get_record(
    request: BatchGetRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchGetRecordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_RECORDS_BATCH_GET
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_record_request_builder() {
        let request = BatchGetRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .record_ids(vec!["rec123".to_string(), "rec456".to_string()])
            .automatic(true)
            .with_shared_url(false)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.record_ids.len(), 2);
        assert_eq!(request.automatic, Some(true));
        assert_eq!(request.with_shared_url, Some(false));
    }
}
