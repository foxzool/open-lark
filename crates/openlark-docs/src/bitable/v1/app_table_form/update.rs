//! 更新表单模块
//!
//! 提供多维表格表单的更新功能。

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

/// 更新表单请求
#[derive(Clone)]
pub struct UpdateFormRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表单的 form_id
    pub form_id: String,
    /// 表单名称
    pub name: Option<String>,
    /// 表单描述
    pub description: Option<String>,
    /// 表单配置
    pub config: Option<serde_json::Value>,
    /// 是否启用表单
    pub is_enabled: Option<bool>,
}

impl UpdateFormRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::PUT,
                BITABLE_V1_FORMS_UPDATE.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            name: None,
            description: None,
            config: None,
            is_enabled: None,
        }
    }

    pub fn builder() -> UpdateFormRequestBuilder {
        UpdateFormRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateFormRequestBuilder {
    request: UpdateFormRequest,
}

impl UpdateFormRequestBuilder {
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

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
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

    pub fn build(self) -> UpdateFormRequest {
        self.request
    }
}

#[derive(Serialize)]
struct UpdateFormRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<bool>,
}

/// 更新表单响应
#[derive(Clone)]
pub struct UpdateFormResponse {
    /// 更新的表单信息
    pub form: UpdateFormResponseData,
}

/// 更新表单响应数据
#[derive(Clone)]
pub struct UpdateFormResponseData {
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
    /// 更新者信息
    pub updater: UpdaterInfo,
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

/// 更新者信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdaterInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

impl ApiResponseTrait for UpdateFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_form_request_builder() {
        let config = json!({
            "theme": "blue",
            "show_submit_button": true,
            "submit_button_text": "更新提交"
        });

        let request = UpdateFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .form_id("frmxxxxxx")
            .name("更新的表单名称")
            .description("更新后的表单描述")
            .config(config.clone())
            .is_enabled(false)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.form_id, "frmxxxxxx");
        assert_eq!(request.name, Some("更新的表单名称".to_string()));
        assert_eq!(request.description, Some("更新后的表单描述".to_string()));
        assert_eq!(request.config, Some(config));
        assert_eq!(request.is_enabled, Some(false));
    }

    #[test]
    fn test_update_form_request_minimal() {
        let request = UpdateFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .form_id("frmxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.form_id, "frmxxxxxx");
        assert_eq!(request.name, None);
        assert_eq!(request.description, None);
        assert_eq!(request.config, None);
        assert_eq!(request.is_enabled, None);
    }

    #[test]
    fn test_update_form_request_body_serialization() {
        let body = UpdateFormRequestBody {
            name: Some("新表单名称".to_string()),
            description: Some("新表单描述".to_string()),
            config: Some(json!({"theme": "dark"})),
            is_enabled: Some(true),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "新表单名称",
            "description": "新表单描述",
            "config": {"theme": "dark"},
            "is_enabled": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_update_form_request_body_empty_serialization() {
        let body = UpdateFormRequestBody {
            name: None,
            description: None,
            config: None,
            is_enabled: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({});

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_updater_info_serialization() {
        let updater = UpdaterInfo {
            user_id: "user_456".to_string(),
            name: "李四".to_string(),
            email: Some("lisi@example.com".to_string()),
        };

        let serialized = serde_json::to_value(&updater).unwrap();
        let expected = json!({
            "user_id": "user_456",
            "name": "李四",
            "email": "lisi@example.com"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_update_form_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = UpdateFormRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.form_id, "");
        assert_eq!(request.name, None);
        assert_eq!(request.description, None);
        assert_eq!(request.config, None);
        assert_eq!(request.is_enabled, None);
    }
}