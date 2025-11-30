#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::Record,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询记录请求
#[derive(Clone)]
pub struct SearchRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
    /// 视图的唯一标识符
    view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    field_names: Option<Vec<String>>,
    /// 排序条件
    sort: Option<Vec<SortCondition>>,
    /// 筛选条件
    filter: Option<FilterInfo>,
    /// 控制是否返回自动计算的字段
    automatic: Option<bool>,
}

/// 排序条件
#[derive(Clone, Serialize, Deserialize)]
pub struct SortCondition {
    /// 字段名称
    pub field_name: String,
    /// 是否倒序排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    /// 条件逻辑连接词: "and" 或 "or"
    pub conjunction: String,
    /// 筛选条件集合
    pub conditions: Vec<FilterCondition>,
}

/// 单个筛选条件
#[derive(Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 筛选条件的左值，值为字段的名称
    pub field_name: String,
    /// 条件运算符
    pub operator: String,
    /// 目标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

impl SearchRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/tables/{}/records/search".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
            view_id: None,
            field_names: None,
            sort: None,
            filter: None,
            automatic: None,
        }
    }

    pub fn builder() -> SearchRecordRequestBuilder {
        SearchRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SearchRecordRequestBuilder {
    request: SearchRecordRequest,
}

impl SearchRecordRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.request.view_id = Some(view_id.into());
        self
    }

    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.request.sort = Some(sort);
        self
    }

    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.request.filter = Some(filter);
        self
    }

    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(self) -> SearchRecordRequest {
        self.request
    }
}

/// 查询记录响应
#[derive(Clone)]
pub struct SearchRecordResponse {
    /// 记录列表
    pub items: Vec<Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for SearchRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct SearchRecordRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<SortCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<FilterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic: Option<bool>,
}

impl FilterInfo {
    /// 创建 AND 条件
    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        Self {
            conjunction: "and".to_string(),
            conditions,
        }
    }

    /// 创建 OR 条件
    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        Self {
            conjunction: "or".to_string(),
            conditions,
        }
    }
}

impl FilterCondition {
    /// 等于
    pub fn equals(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "is".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 不等于
    pub fn not_equals(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "isNot".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 包含
    pub fn contains(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "contains".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 不包含
    pub fn not_contains(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "doesNotContain".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 为空
    pub fn is_empty(field_name: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "isEmpty".to_string(),
            value: None,
        }
    }

    /// 不为空
    pub fn is_not_empty(field_name: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "isNotEmpty".to_string(),
            value: None,
        }
    }
}

/// 查询记录
pub async fn search_record(
    request: SearchRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<SearchRecordResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_RECORDS_SEARCH
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = SearchRecordRequestBody {
        page_size: request.page_size,
        page_token: request.page_token,
        view_id: request.view_id,
        field_names: request.field_names,
        sort: request.sort,
        filter: request.filter,
        automatic: request.automatic,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<SearchRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_record_request_builder() {
        let filter = FilterInfo::and(vec![
            FilterCondition::equals("状态", "进行中"),
            FilterCondition::is_not_empty("标题"),
        ]);

        let sort = vec![SortCondition {
            field_name: "创建时间".to_string(),
            desc: Some(true),
        }];

        let request = SearchRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(20)
            .filter(filter)
            .sort(sort)
            .field_names(vec!["标题".to_string(), "状态".to_string()])
            .automatic(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert!(request.filter.is_some());
        assert!(request.sort.is_some());
        assert!(request.field_names.is_some());
        assert_eq!(request.automatic, Some(true));
    }

    #[test]
    fn test_filter_conditions() {
        let filter = FilterInfo::or(vec![
            FilterCondition::equals("优先级", "高"),
            FilterCondition::contains("标题", "紧急"),
            FilterCondition::is_empty("完成时间"),
        ]);

        assert_eq!(filter.conjunction, "or");
        assert_eq!(filter.conditions.len(), 3);
        assert_eq!(filter.conditions[0].operator, "is");
        assert_eq!(filter.conditions[1].operator, "contains");
        assert_eq!(filter.conditions[2].operator, "isEmpty");
    }

    #[test]
    fn test_sort_condition_serialization() {
        let sort = SortCondition {
            field_name: "创建时间".to_string(),
            desc: Some(true),
        };

        let serialized = serde_json::to_value(&sort).unwrap();
        let expected = serde_json::json!({
            "field_name": "创建时间",
            "desc": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_search_record_request_body_serialization() {
        let filter = FilterInfo::and(vec![
            FilterCondition::equals("状态", "完成"),
            FilterCondition::not_equals("优先级", "低"),
        ]);

        let body = SearchRecordRequestBody {
            page_size: Some(20),
            page_token: Some("token123".to_string()),
            filter: Some(filter),
            automatic: Some(true),
            ..Default::default()
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "page_size": 20,
            "page_token": "token123",
            "filter": {
                "conjunction": "and",
                "conditions": [
                    {
                        "field_name": "状态",
                        "operator": "is",
                        "value": ["完成"]
                    },
                    {
                        "field_name": "优先级",
                        "operator": "isNot",
                        "value": ["低"]
                    }
                ]
            },
            "automatic": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_search_record_request_minimal() {
        let request = SearchRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.view_id.is_none());
        assert!(request.field_names.is_none());
        assert!(request.sort.is_none());
        assert!(request.filter.is_none());
        assert!(request.automatic.is_none());
    }

    #[test]
    fn test_page_size_limit() {
        let request = SearchRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_search_record_request_builder_chaining() {
        let filter = FilterInfo::and(vec![
            FilterCondition::contains("标题", "测试"),
        ]);

        let sort = vec![SortCondition {
            field_name: "更新时间".to_string(),
            desc: Some(false),
        }];

        let request = SearchRecordRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .user_id_type("open_id")
            .page_token("page123")
            .page_size(50)
            .view_id("view123")
            .field_names(vec!["field1".to_string(), "field2".to_string()])
            .sort(sort)
            .filter(filter)
            .automatic(false)
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.page_token, Some("page123".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.view_id, Some("view123".to_string()));
        assert_eq!(request.automatic, Some(false));
    }

    #[test]
    fn test_search_record_response_trait() {
        assert_eq!(SearchRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_search_record_response() {
        let response = SearchRecordResponse {
            items: vec![
                Record {
                    record_id: Some("rec123".to_string()),
                    fields: std::collections::HashMap::from([
                        ("标题".to_string(), serde_json::json!("测试记录")),
                    ]),
                    created_by: None,
                    created_time: None,
                    last_modified_by: None,
                    last_modified_time: None,
                },
            ],
            has_more: true,
            page_token: Some("next_page_token".to_string()),
            total: 100,
        };

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].record_id, Some("rec123".to_string()));
        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.total, 100);
    }

    #[test]
    fn test_filter_condition_methods() {
        let equals = FilterCondition::equals("status", "active");
        assert_eq!(equals.field_name, "status");
        assert_eq!(equals.operator, "is");
        assert_eq!(equals.value, Some(vec!["active"]));

        let not_equals = FilterCondition::not_equals("priority", "low");
        assert_eq!(not_equals.operator, "isNot");

        let contains = FilterCondition::contains("title", "test");
        assert_eq!(contains.operator, "contains");

        let not_contains = FilterCondition::not_contains("desc", "temp");
        assert_eq!(not_contains.operator, "doesNotContain");

        let is_empty = FilterCondition::is_empty("field");
        assert_eq!(is_empty.operator, "isEmpty");
        assert!(is_empty.value.is_none());

        let is_not_empty = FilterCondition::is_not_empty("field");
        assert_eq!(is_not_empty.operator, "isNotEmpty");
        assert!(is_not_empty.value.is_none());
    }

    #[test]
    fn test_search_record_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = SearchRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.view_id.is_none());
        assert!(request.field_names.is_none());
        assert!(request.sort.is_none());
        assert!(request.filter.is_none());
        assert!(request.automatic.is_none());
    }
}