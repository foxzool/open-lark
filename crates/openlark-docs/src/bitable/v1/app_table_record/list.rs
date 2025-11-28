//! 列出记录模块
//!
//! 提供多维表格记录的列表查询功能，支持分页、筛选和排序。

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::AppTableRecordService;

/// 列出记录请求
#[derive(Clone)]
pub struct ListRecordRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 视图的唯一标识符
    pub view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    pub field_names: Option<Vec<String>>,
    /// 排序条件
    pub sort: Option<Vec<SortCondition>>,
    /// 筛选条件
    pub filter: Option<FilterInfo>,
    /// 控制是否返回自动计算的字段
    pub automatic: Option<bool>,
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

impl ListRecordRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                BITABLE_V1_RECORDS_LIST.to_string(),
            ),
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

    pub fn builder() -> ListRecordRequestBuilder {
        ListRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListRecordRequestBuilder {
    request: ListRecordRequest,
}

impl ListRecordRequestBuilder {
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
        self.request.page_size = Some(page_size);
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

    pub fn build(self) -> ListRecordRequest {
        self.request
    }
}

/// 列出记录响应
#[derive(Clone)]
pub struct ListRecordResponse {
    /// 记录列表
    pub items: Vec<openlark_core::service::bitable::v1::Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for ListRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

    /// 大于
    pub fn greater_than(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "isGreater".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 小于
    pub fn less_than(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: "isLess".to_string(),
            value: Some(vec![value.to_string()]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_list_record_request_builder() {
        let filter = FilterInfo::and(vec![
            FilterCondition::equals("状态", "进行中"),
            FilterCondition::is_not_empty("标题"),
        ]);

        let sort = vec![SortCondition {
            field_name: "创建时间".to_string(),
            desc: Some(true),
        }];

        let request = ListRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(20)
            .filter(filter)
            .sort(sort)
            .field_names(vec!["标题".to_string(), "状态".to_string()])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert!(request.filter.is_some());
        assert!(request.sort.is_some());
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
        let expected = json!({
            "field_name": "创建时间",
            "desc": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_filter_info_serialization() {
        let filter = FilterInfo::and(vec![
            FilterCondition::equals("状态", "完成"),
            FilterCondition::not_equals("优先级", "低"),
        ]);

        let serialized = serde_json::to_value(&filter).unwrap();
        let expected = json!({
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
        });

        assert_eq!(serialized, expected);
    }
}