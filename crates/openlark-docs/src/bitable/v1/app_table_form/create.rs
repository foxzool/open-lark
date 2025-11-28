//! 创建表单模块
//!
//! 提供多维表格表单的创建功能。

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
use super::AppTableFormService;

/// 创建表单请求
#[derive(Clone)]
pub struct CreateFormRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表单名称
    pub name: String,
    /// 表单描述
    pub description: Option<String>,
    /// 表单配置
    pub config: Option<serde_json::Value>,
    /// 是否启用表单
    pub is_enabled: Option<bool>,
}

impl CreateFormRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::POST,
                BITABLE_V1_FORMS_CREATE.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            name: String::new(),
            description: None,
            config: None,
            is_enabled: Some(true),
        }
    }

    pub fn builder() -> CreateFormRequestBuilder {
        CreateFormRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFormRequestBuilder {
    request: CreateFormRequest,
}

impl CreateFormRequestBuilder {
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn config(mut self, config: serde_json::Value) -> Self {
        self.request.config = Some(config);
        self
    }

    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.request.is_enabled = Some(is_enabled);
        self
    }

    pub fn build(self) -> CreateFormRequest {
        self.request
    }
}

#[derive(Serialize)]
struct CreateFormRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<bool>,
}

/// 创建表单响应
#[derive(Clone)]
pub struct CreateFormResponse {
    /// 创建的表单信息
    pub form: CreateFormResponseData,
}

/// 创建表单响应数据
#[derive(Clone)]
pub struct CreateFormResponseData {
    /// 表单的form_id
    pub form_id: String,
    /// 表单名称
    pub name: String,
    /// 表单描述
    pub description: Option<String>,
    /// 表单配置
    pub config: Option<serde_json::Value>,
    /// 是否启用
    pub is_enabled: bool,
    /// 创建时间
    pub created_time: String,
    /// 更新时间
    pub updated_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
}

/// 创建者信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

impl ApiResponseTrait for CreateFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_form_request_builder() {
        let config = json!({
            "theme": "default",
            "show_submit_button": true,
            "submit_button_text": "提交",
            "thank_you_message": "感谢您的提交！"
        });

        let request = CreateFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .name("用户反馈表单")
            .description("收集用户反馈信息")
            .config(config.clone())
            .is_enabled(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, "用户反馈表单");
        assert_eq!(request.description, Some("收集用户反馈信息".to_string()));
        assert_eq!(request.config, Some(config));
        assert_eq!(request.is_enabled, Some(true));
    }

    #[test]
    fn test_create_form_request_minimal() {
        let request = CreateFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .name("简单表单")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, "简单表单");
        assert_eq!(request.description, None);
        assert_eq!(request.config, None);
        assert_eq!(request.is_enabled, Some(true)); // 默认启用
    }

    #[test]
    fn test_create_form_request_body_serialization() {
        let body = CreateFormRequestBody {
            name: "测试表单".to_string(),
            description: Some("测试表单描述".to_string()),
            config: Some(json!({"theme": "dark"})),
            is_enabled: Some(false),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "测试表单",
            "description": "测试表单描述",
            "config": {"theme": "dark"},
            "is_enabled": false
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_form_request_body_minimal_serialization() {
        let body = CreateFormRequestBody {
            name: "最小表单".to_string(),
            description: None,
            config: None,
            is_enabled: Some(true),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "最小表单",
            "is_enabled": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_form_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateFormRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.name, "");
        assert_eq!(request.description, None);
        assert_eq!(request.config, None);
        assert_eq!(request.is_enabled, Some(true));
    }

    #[test]
    fn test_creator_info_serialization() {
        let creator = CreatorInfo {
            user_id: "user_123".to_string(),
            name: "张三".to_string(),
            email: Some("zhangsan@example.com".to_string()),
        };

        let serialized = serde_json::to_value(&creator).unwrap();
        let expected = json!({
            "user_id": "user_123",
            "name": "张三",
            "email": "zhangsan@example.com"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_complex_config_serialization() {
        let complex_config = json!({
            "theme": {
                "primary_color": "#1890ff",
                "background_color": "#ffffff"
            },
            "fields": [
                {
                    "field_id": "fld_name",
                    "label": "姓名",
                    "required": true,
                    "type": "text"
                },
                {
                    "field_id": "fld_email",
                    "label": "邮箱",
                    "required": true,
                    "type": "email"
                }
            ],
            "settings": {
                "show_progress": true,
                "allow_multiple_submissions": false,
                "auto_save": true
            }
        });

        let body = CreateFormRequestBody {
            name: "复杂配置表单".to_string(),
            description: Some("带有复杂配置的表单".to_string()),
            config: Some(complex_config.clone()),
            is_enabled: Some(true),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "复杂配置表单",
            "description": "带有复杂配置的表单",
            "config": complex_config,
            "is_enabled": true
        });

        assert_eq!(serialized, expected);
    }
}