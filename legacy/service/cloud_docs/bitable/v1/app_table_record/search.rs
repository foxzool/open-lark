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

/// 查询记录请求
#[derive(Debug, Serialize, Default, Clone)]
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
#[derive(Debug, Serialize, Default, Clone)]
pub struct SortCondition {
    /// 字段名称
    pub field_name: String,
    /// 是否倒序排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Debug, Serialize, Default, Clone)]
pub struct FilterInfo {
    /// 条件逻辑连接词: "and" 或 "or"
    pub conjunction: String,
    /// 筛选条件集合
    pub conditions: Vec<FilterCondition>,
}

/// 单个筛选条件
#[derive(Debug, Serialize, Default, Clone)]
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
    pub fn builder() -> SearchRecordRequestBuilder {
        SearchRecordRequestBuilder::default()
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
pub struct SearchRecordRequestBuilder {
    request: SearchRecordRequest,
}

impl SearchRecordRequestBuilder {
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

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 视图的唯一标识符
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = Some(view_id.to_string());
        self
    }

    /// 字段名称
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    /// 排序条件
    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.request.sort = Some(sort);
        self
    }

    /// 筛选条件
    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.request.filter = Some(filter);
        self
    }

    /// 控制是否返回自动计算的字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(mut self) -> SearchRecordRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 应用ExecutableBuilder trait到SearchRecordRequestBuilder
crate::impl_executable_builder_owned!(
    SearchRecordRequestBuilder,
    super::AppTableRecordService,
    SearchRecordRequest,
    BaseResponse<SearchRecordResponse>,
    search
);

/// 查询记录响应
#[derive(Debug, Deserialize)]
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

/// 查询记录
pub async fn search_record(
    request: SearchRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<SearchRecordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_RECORDS_SEARCH
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
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
#[allow(unused_variables, unused_unsafe)]
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
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert!(request.filter.is_some());
        assert!(request.sort.is_some());
    }
}
