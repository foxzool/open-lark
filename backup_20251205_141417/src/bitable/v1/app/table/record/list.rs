//! Bitable V1 列出记录API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 列出记录请求
#[derive(Debug, Clone)]
pub struct ListRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
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

impl ListRecordRequest {
    /// 创建列出记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
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

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = Some(view_id);
        self
    }

    /// 设置字段名称
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.field_names = Some(field_names);
        self
    }

    /// 设置排序条件
    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 设置筛选条件
    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.filter = Some(filter);
        self
    }

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.automatic = Some(automatic);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "分页大小必须大于0"));
            }
        }

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", self.app_token, self.table_id);

        // 创建API请求
        let mut api_request: ApiRequest<ListRecordResponse> =
            ApiRequest::get(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref view_id) = self.view_id {
            api_request = api_request.query("view_id", view_id);
        }

        if let Some(ref field_names) = self.field_names {
            api_request = api_request.query("field_names", &field_names.join(","));
        }

        if let Some(automatic) = self.automatic {
            api_request = api_request.query("automatic", &automatic.to_string());
        }

        // 构建请求体
        let request_body = ListRecordRequestBody {
            sort: self.sort,
            filter: self.filter,
        };

        // 设置请求体
        api_request = api_request.body(RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出记录Builder
pub struct ListRecordRequestBuilder {
    request: ListRecordRequest,
}

impl ListRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListRecordRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 设置字段名称
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request = self.request.field_names(field_names);
        self
    }

    /// 设置排序条件
    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.request = self.request.sort(sort);
        self
    }

    /// 设置筛选条件
    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.request = self.request.filter(filter);
        self
    }

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request = self.request.automatic(automatic);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListRecordRequest {
        self.request
    }
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    /// 记录ID
    pub record_id: String,
    /// 字段数据
    pub fields: serde_json::Value,
    /// 创建时间
    pub created_time: String,
    /// 最后更新时间
    pub last_modified_time: String,
}

/// 排序条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortCondition {
    /// 字段名称
    pub field_name: String,
    /// 是否倒序排序
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterInfo {
    /// 条件逻辑连接词: and 或 or
    pub conjunction: String,
    /// 筛选条件集合
    pub conditions: Vec<FilterCondition>,
}

/// 单个筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    /// 筛选条件的左值，值为字段的名称
    pub field_name: String,
    /// 条件运算符
    pub operator: String,
    /// 目标值
    pub value: Option<Vec<String>>,
}

/// 列出记录请求体（内部使用）
#[derive(Debug, Serialize)]
struct ListRecordRequestBody {
    sort: Option<Vec<SortCondition>>,
    filter: Option<FilterInfo>,
}

/// 列出记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRecordData {
    /// 记录列表
    pub items: Vec<Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

/// 列出记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRecordResponse {
    /// 记录列表数据
    pub data: ListRecordData,
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
