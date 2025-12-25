/// Bitable 检索记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/get
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 获取记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
    text_field_as_array: Option<bool>,
    user_id_type: Option<String>,
    display_formula_ref: Option<bool>,
    with_shared_url: Option<bool>,
    automatic_fields: Option<bool>,
}

impl GetRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            text_field_as_array: None,
            user_id_type: None,
            display_formula_ref: None,
            with_shared_url: None,
            automatic_fields: None,
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

    /// 多行文本字段数据是否以数组形式返回
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 控制公式、查找引用是否显示完整原样的返回结果
    pub fn display_formula_ref(mut self, display_formula_ref: bool) -> Self {
        self.display_formula_ref = Some(display_formula_ref);
        self
    }

    /// 控制是否返回该记录的链接（record_url）
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.with_shared_url = Some(with_shared_url);
        self
    }

    /// 控制是否返回自动计算字段
    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn execute(self) -> SDKResult<GetRecordResponse> {
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
            BitableApiV1::RecordGet(self.app_token.clone(), self.table_id.clone(), self.record_id);

        let mut api_request: ApiRequest<GetRecordResponse> = ApiRequest::get(&api_endpoint.to_url());

        api_request = api_request.query_opt(
            "text_field_as_array",
            self.text_field_as_array.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt(
            "display_formula_ref",
            self.display_formula_ref.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt(
            "with_shared_url",
            self.with_shared_url.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt(
            "automatic_fields",
            self.automatic_fields.map(|v| v.to_string()),
        );

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 获取记录 Builder
pub struct GetRecordRequestBuilder {
    request: GetRecordRequest,
}

impl GetRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: GetRecordRequest::new(config),
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

    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request = self.request.text_field_as_array(text_field_as_array);
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn display_formula_ref(mut self, display_formula_ref: bool) -> Self {
        self.request = self.request.display_formula_ref(display_formula_ref);
        self
    }

    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.request = self.request.with_shared_url(with_shared_url);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.request = self.request.automatic_fields(automatic_fields);
        self
    }

    pub fn build(self) -> GetRecordRequest {
        self.request
    }
}

/// 获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordResponse {
    /// 记录
    pub record: Record,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
