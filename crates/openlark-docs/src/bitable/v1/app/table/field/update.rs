//! Bitable V1 更新字段API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 重用Field类型
pub use super::create::{Field, FieldProperty, FieldType};

/// 更新字段请求
#[derive(Debug, Clone)]
pub struct UpdateFieldRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<UpdateFieldResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 字段的唯一标识符
    field_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 多维表格字段名
    field_name: Option<String>,
    /// 多维表格字段类型
    field_type: Option<FieldType>,
    /// 字段属性
    property: Option<FieldProperty>,
    /// 字段的描述
    description: Option<String>,
    /// 字段在界面上的展示类型
    ui_type: Option<String>,
}

impl UpdateFieldRequest {
    /// 创建更新字段请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::put(""),
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
            user_id_type: None,
            field_name: None,
            field_type: None,
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

    /// 设置字段ID
    pub fn field_id(mut self, field_id: String) -> Self {
        self.field_id = field_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置字段名称
    pub fn field_name(mut self, field_name: String) -> Self {
        self.field_name = Some(field_name);
        self
    }

    /// 设置字段类型
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.field_type = Some(field_type);
        self
    }

    /// 设置字段属性
    pub fn property(mut self, property: FieldProperty) -> Self {
        self.property = Some(property);
        self
    }

    /// 设置字段描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置界面类型
    pub fn ui_type(mut self, ui_type: String) -> Self {
        self.ui_type = Some(ui_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateFieldResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.field_id.trim().is_empty() {
            return Err(validation_error("field_id", "字段ID不能为空"));
        }

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}", self.app_token, self.table_id, self.field_id);

        // 创建API请求
        let mut api_request: ApiRequest<UpdateFieldResponse> =
            ApiRequest::put(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 设置请求体
        let body = UpdateFieldRequestBody {
            field_name: self.field_name,
            r#type: self.field_type,
            property: self.property,
            description: self.description,
            ui_type: self.ui_type,
        };

        // 设置请求体
        api_request = api_request.body(RequestData::Binary(serde_json::to_vec(&body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 更新字段Builder
pub struct UpdateFieldRequestBuilder {
    request: UpdateFieldRequest,
}

impl UpdateFieldRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateFieldRequest::new(config),
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

    /// 设置字段ID
    pub fn field_id(mut self, field_id: String) -> Self {
        self.request = self.request.field_id(field_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
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
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置界面类型
    pub fn ui_type(mut self, ui_type: String) -> Self {
        self.request = self.request.ui_type(ui_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateFieldRequest {
        self.request
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateFieldRequestBody {
    field_name: Option<String>,
    r#type: Option<FieldType>,
    property: Option<FieldProperty>,
    description: Option<String>,
    ui_type: Option<String>,
}

/// 更新字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateFieldResponse {
    /// 字段信息
    pub data: Field,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
