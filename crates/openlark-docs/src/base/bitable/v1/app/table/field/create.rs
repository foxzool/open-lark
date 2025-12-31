/// Bitable 新增字段
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 字段类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FieldType {
    Text = 1,
    Number = 2,
    SingleSelect = 3,
    MultiSelect = 4,
    DateTime = 5,
    Checkbox = 7,
    User = 11,
    PhoneNumber = 13,
    Url = 15,
    Attachment = 17,
    Link = 18,
    Formula = 20,
    DuplexLink = 21,
    Location = 22,
    GroupChat = 23,
    CreatedTime = 1001,
    ModifiedTime = 1002,
    CreatedUser = 1003,
    ModifiedUser = 1004,
    AutoNumber = 1005,
}

/// 字段属性
pub type FieldProperty = Value;

/// 创建字段请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CreateFieldRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 多维表格字段名
    field_name: String,
    /// 多维表格字段类型
    r#type: FieldType,
    /// 字段属性
    property: Option<FieldProperty>,
    /// 字段的描述
    description: Option<Value>,
    /// 字段在界面上的展示类型
    ui_type: Option<String>,
}

impl CreateFieldRequest {
    /// 创建字段请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            client_token: None,
            field_name: String::new(),
            r#type: FieldType::Text,
            property: None,
            description: None,
            ui_type: None,
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

    /// 设置客户端token
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 设置字段名称
    pub fn field_name(mut self, field_name: String) -> Self {
        self.field_name = field_name;
        self
    }

    /// 设置字段类型
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.r#type = field_type;
        self
    }

    /// 设置字段属性
    pub fn property(mut self, property: FieldProperty) -> Self {
        self.property = Some(property);
        self
    }

    /// 设置字段描述
    pub fn description(mut self, description: Value) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置界面类型
    pub fn ui_type(mut self, ui_type: String) -> Self {
        self.ui_type = Some(ui_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateFieldResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.field_name.trim().is_empty() {
            return Err(validation_error("field_name", "字段名称不能为空"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FieldCreate(self.app_token.clone(), self.table_id.clone());

        // 构建请求体
        let body = CreateFieldRequestBody {
            field_name: self.field_name,
            r#type: self.r#type,
            property: self.property,
            description: self.description,
            ui_type: self.ui_type,
        };

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateFieldResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 构建查询参数
        api_request = api_request.query_opt("client_token", self.client_token);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 创建字段Builder
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest,
}

impl CreateFieldRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateFieldRequest::new(config),
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

    /// 设置客户端token
    pub fn client_token(mut self, client_token: String) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    /// 设置字段名称
    pub fn field_name(mut self, field_name: String) -> Self {
        self.request = self.request.field_name(field_name);
        self
    }

    /// 设置字段类型
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.request = self.request.field_type(field_type);
        self
    }

    /// 设置字段属性
    pub fn property(mut self, property: FieldProperty) -> Self {
        self.request = self.request.property(property);
        self
    }

    /// 设置字段描述
    pub fn description(mut self, description: Value) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置界面类型
    pub fn ui_type(mut self, ui_type: String) -> Self {
        self.request = self.request.ui_type(ui_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateFieldRequest {
        self.request
    }
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: i32,
    /// 字段属性，不同字段类型对应不同的属性结构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 字段描述（可能是 string/array/object，受 text_field_as_array 影响）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Value>,
    /// 是否是索引列
    pub is_primary: bool,
    /// 字段 ID
    pub field_id: String,
    /// 字段在界面上的展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_type: Option<String>,
    /// 是否是隐藏字段
    pub is_hidden: bool,
}

/// 请求体结构
#[derive(Serialize)]
struct CreateFieldRequestBody {
    field_name: String,
    r#type: FieldType,
    property: Option<FieldProperty>,
    description: Option<Value>,
    ui_type: Option<String>,
}

/// 创建字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFieldResponse {
    /// 字段信息
    pub field: Field,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
