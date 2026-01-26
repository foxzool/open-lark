//! Bitable 更新字段
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 重用Field类型
pub use super::create::{Field, FieldProperty, FieldType};

/// 更新字段请求
///
/// 用于更新多维表格数据表中的指定字段。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token
/// - `table_id`: 数据表的 table_id
/// - `field_id`: 字段的唯一标识符
/// - `field_name`: 多维表格字段名
/// - `type`: 多维表格字段类型
/// - `property`: 字段属性（可选）
/// - `description`: 字段的描述（可选）
/// - `ui_type`: 字段在界面上的展示类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::base::bitable::v1::app::table::field::update::{UpdateFieldRequest, FieldType};
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = UpdateFieldRequest::new(config)
///     .app_token("app_token_xyz".to_string())
///     .table_id("table_id_xyz".to_string())
///     .field_id("field_id_xyz".to_string())
///     .field_name("更新后的字段名".to_string())
///     .field_type(FieldType::Text);
/// ```
#[derive(Debug, Clone)]
pub struct UpdateFieldRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 字段的唯一标识符
    field_id: String,
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

impl UpdateFieldRequest {
    /// 创建更新字段请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
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

    /// 设置字段ID
    pub fn field_id(mut self, field_id: String) -> Self {
        self.field_id = field_id;
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
    pub async fn execute(self) -> SDKResult<UpdateFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateFieldResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        validate_required!(self.field_id.trim(), "field_id");

        validate_required!(self.field_name.trim(), "field_name");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FieldUpdate(
            self.app_token.clone(),
            self.table_id.clone(),
            self.field_id.clone(),
        );

        // 设置请求体
        let body = UpdateFieldRequestBody {
            field_name: self.field_name,
            r#type: self.r#type,
            property: self.property,
            description: self.description,
            ui_type: self.ui_type,
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateFieldResponse> =
            ApiRequest::put(&api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateFieldRequestBody {
    field_name: String,
    r#type: FieldType,
    property: Option<FieldProperty>,
    description: Option<Value>,
    ui_type: Option<String>,
}

/// 更新字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateFieldResponse {
    /// 字段信息
    pub field: Field,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = UpdateFieldRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .field_id("field_id".to_string())
            .field_name("字段名".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_table_id() {
        let config = Config::default();
        let request = UpdateFieldRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .field_id("field_id".to_string())
            .field_name("字段名".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("table_id"));
    }

    #[test]
    fn test_empty_field_id() {
        let config = Config::default();
        let request = UpdateFieldRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .field_id("".to_string())
            .field_name("字段名".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("field_id"));
    }

    #[test]
    fn test_empty_field_name() {
        let config = Config::default();
        let request = UpdateFieldRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .field_id("field_id".to_string())
            .field_name("".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("field_name"));
    }

    #[test]
    fn test_update_field_request_builder() {
        let config = Config::default();
        let request = UpdateFieldRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .field_id("field_id".to_string())
            .field_name("更新后的字段名".to_string())
            .field_type(FieldType::Number);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.field_id, "field_id");
        assert_eq!(request.field_name, "更新后的字段名");
        assert_eq!(request.r#type, FieldType::Number);
    }
}
