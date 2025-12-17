/// Bitable 创建数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 新增数据表请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CreateTableRequest {
    api_request: ApiRequest<CreateTableResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息
    table: TableData,
    /// 配置信息
    config: Config,
}

/// 创建数据表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTableData {
    /// 数据表信息
    pub table: TableData,
}

/// 创建数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTableResponse {
    /// 创建数据表数据
    pub data: CreateTableData,
}

impl ApiResponseTrait for CreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateTableRequest {
    /// 创建新增数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post(""), // 占位符，将在execute方法中使用enum+builder系统
            app_token: String::new(),
            table: TableData::default(),
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表信息
    pub fn table(mut self, table: TableData) -> Self {
        self.table = table;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTableResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table.name.trim().is_empty() {
            return Err(validation_error("name", "数据表名称不能为空"));
        }

        if self.table.name.len() > 100 {
            return Err(validation_error("name", "数据表名称长度不能超过100个字符"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TableCreate(self.app_token.clone());

        // 构建请求体
        let request_body = CreateTableRequestBody { table: self.table };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateTableResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 创建数据表Builder
pub struct CreateTableRequestBuilder {
    request: CreateTableRequest,
}

impl CreateTableRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateTableRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表信息
    pub fn table(mut self, table: TableData) -> Self {
        self.request = self.request.table(table);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateTableRequest {
        self.request
    }
}

/// 数据表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableData {
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图名称，不填则默认为数据表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view_name: Option<String>,
    /// 数据表初始字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableField>>,
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            name: String::new(),
            default_view_name: None,
            fields: None,
        }
    }
}

impl TableData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default_view_name: None,
            fields: None,
        }
    }

    /// 设置默认视图名称
    pub fn with_default_view_name(mut self, view_name: impl Into<String>) -> Self {
        self.default_view_name = Some(view_name.into());
        self
    }

    /// 设置初始字段
    pub fn with_fields(mut self, fields: Vec<TableField>) -> Self {
        self.fields = Some(fields);
        self
    }
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableField {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: i32,
    /// 字段属性，不同字段类型对应不同的属性结构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

impl TableField {
    pub fn new(name: impl Into<String>, field_type: i32) -> Self {
        Self {
            field_name: name.into(),
            field_type,
            property: None,
        }
    }

    /// 创建文本字段
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, 1) // 文本
    }

    /// 创建数字字段
    pub fn number(name: impl Into<String>) -> Self {
        Self::new(name, 2) // 数字
    }

    /// 创建单选字段
    pub fn single_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 3, // 单选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建多选字段
    pub fn multi_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 4, // 多选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建日期字段
    pub fn date(name: impl Into<String>) -> Self {
        Self::new(name, 5) // 日期
    }
}

/// 创建数据表请求体（内部使用）
#[derive(Serialize)]
#[allow(dead_code)]
struct CreateTableRequestBody {
    table: TableData,
}
