#!/bin/bash

# Fix user.rs file with a clean, minimal implementation

USER_FILE="/Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src/contact/v3/user.rs"

# Create a clean, working version
cat > "${USER_FILE}.clean" << 'EOF'
use open_lark_core::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
};
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 用户管理服务
pub struct UserService {
    config: Config,
}

impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户
    pub async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> open_lark_core::core::SDKResult<CreateUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_USERS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个用户信息
    pub async fn get(
        &self,
        user_id: &str,
        _req: &GetUserRequest,
    ) -> open_lark_core::core::SDKResult<GetUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                open_lark_core::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取用户列表
    pub async fn list(&self, _req: &ListUsersRequest) -> open_lark_core::core::SDKResult<ListUsersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_USERS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<ListUsersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新用户
    pub async fn patch(
        &self,
        user_id: &str,
        req: &PatchUserRequest,
    ) -> open_lark_core::core::SDKResult<PatchUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                open_lark_core::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除用户
    pub async fn delete(
        &self,
        user_id: &str,
        _req: &DeleteUserRequest,
    ) -> open_lark_core::core::SDKResult<DeleteUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                open_lark_core::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateUserResponse {
    pub user: User,
}

impl ApiResponseTrait for CreateUserResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUserRequest {
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchUserResponse {
    pub user: User,
}

impl ApiResponseTrait for PatchUserResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserResponse {
    pub user: User,
}

impl ApiResponseTrait for GetUserResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListUsersResponse {
    pub items: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUsersResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteUserResponse {}

impl ApiResponseTrait for DeleteUserResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}
EOF

# Replace the original file
mv "${USER_FILE}.clean" "$USER_FILE"

echo "Fixed user.rs file with clean implementation"