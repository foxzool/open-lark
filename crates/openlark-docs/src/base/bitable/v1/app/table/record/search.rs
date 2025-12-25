/// Bitable 查询记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search
/// doc: https://open.feishu.cn/document/docs/bitable-v1/app-table-record/search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 查询记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SearchRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    user_id_type: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
    body: SearchRecordRequestBody,
}

impl SearchRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
            body: SearchRecordRequestBody::default(),
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

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(500));
        self
    }

    /// 视图 ID（当 filter 或 sort 不为空时，view_id 会被忽略）
    pub fn view_id(mut self, view_id: String) -> Self {
        self.body.view_id = Some(view_id);
        self
    }

    /// 指定返回字段名
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.body.field_names = Some(field_names);
        self
    }

    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.body.sort = Some(sort);
        self
    }

    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.body.filter = Some(filter);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.body.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchRecordResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "page_size 必须大于 0"));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RecordSearch(self.app_token.clone(), self.table_id.clone());

        let mut api_request: ApiRequest<SearchRecordResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&self.body)?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("page_token", self.page_token);
        api_request =
            api_request.query_opt("page_size", self.page_size.map(|v| v.to_string()));

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 查询记录 Builder
pub struct SearchRecordRequestBuilder {
    request: SearchRecordRequest,
}

impl SearchRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: SearchRecordRequest::new(config),
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

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request = self.request.field_names(field_names);
        self
    }

    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.request = self.request.sort(sort);
        self
    }

    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.request = self.request.filter(filter);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.request = self.request.automatic_fields(automatic_fields);
        self
    }

    pub fn build(self) -> SearchRecordRequest {
        self.request
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
struct SearchRecordRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<SortCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<FilterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_fields: Option<bool>,
}

/// 排序条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FilterCondition>>,
}

/// 单个筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    pub field_name: String,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// 查询记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRecordResponse {
    pub items: Vec<Record>,
    pub has_more: bool,
    pub page_token: Option<String>,
    pub total: i32,
}

impl ApiResponseTrait for SearchRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
