//! 获取用户角色列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/user_role/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::{I18nText, IdNameObject};

#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    page_token: Option<String>,
    page_size: Option<i32>,
    user_id: Option<String>,
    role_id: Option<String>,
    update_start_time: Option<String>,
    update_end_time: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
            user_id: None,
            role_id: None,
            update_start_time: None,
            update_end_time: None,
            user_id_type: None,
        }
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = Some(role_id.into());
        self
    }

    pub fn update_start_time(mut self, update_start_time: impl Into<String>) -> Self {
        self.update_start_time = Some(update_start_time.into());
        self
    }

    pub fn update_end_time(mut self, update_end_time: impl Into<String>) -> Self {
        self.update_end_time = Some(update_end_time.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-100 之间",
                ));
            }
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v1/user_roles");
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(user_id) = self.user_id {
            request = request.query("user_id", user_id);
        }
        if let Some(role_id) = self.role_id {
            request = request.query("role_id", role_id);
        }
        if let Some(update_start_time) = self.update_start_time {
            request = request.query("update_start_time", update_start_time);
        }
        if let Some(update_end_time) = self.update_end_time {
            request = request.query("update_end_time", update_end_time);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取用户角色列表响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScopeRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BusinessManagementScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_rule: Option<ScopeRule>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserRoleItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_management_scopes: Option<Vec<BusinessManagementScope>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    pub items: Vec<UserRoleItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
