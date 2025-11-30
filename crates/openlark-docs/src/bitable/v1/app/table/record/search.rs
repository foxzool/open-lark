
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询记录请求
#[derive(Clone)]
pub struct SearchRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
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
    #[serde(skip_serializing_if = Option::is_none)]
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    /// 条件逻辑连接词: and 或 or
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
    #[serde(skip_serializing_if = Option::is_none)]
    pub value: Option<Vec<String>>,
}

impl SearchRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/records/search).config(config)),
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
    #[serde(skip_serializing_if = Option::is_none)]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = Option::is_none)]
    page_token: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    view_id: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = Option::is_none)]
    sort: Option<Vec<SortCondition>>,
    #[serde(skip_serializing_if = Option::is_none)]
    filter: Option<FilterInfo>,
    #[serde(skip_serializing_if = Option::is_none)]
    automatic: Option<bool>,
}

impl FilterInfo {
    /// 创建 AND 条件
    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        Self {
            conjunction: and.to_string(),
            conditions,
        }
    }

    /// 创建 OR 条件
    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        Self {
            conjunction: or.to_string(),
            conditions,
        }
    }
}

impl FilterCondition {
    /// 等于
    pub fn equals(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: is.to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 不等于
    pub fn not_equals(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: isNot.to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 包含
    pub fn contains(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: contains.to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 不包含
    pub fn not_contains(field_name: impl ToString, value: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: doesNotContain.to_string(),
            value: Some(vec![value.to_string()]),
        }
    }

    /// 为空
    pub fn is_empty(field_name: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: isEmpty.to_string(),
            value: None,
        }
    }

    /// 不为空
    pub fn is_not_empty(field_name: impl ToString) -> Self {
        Self {
            field_name: field_name.to_string(),
            operator: isNotEmpty.to_string(),
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
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
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

