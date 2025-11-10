#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! 认证服务 V1 版本
//!
//! 提供完整的飞书开放平台认证功能，包括：
//! - 用户信息获取和管理
//! - 应用访问令牌（App Access Token）管理
//! - 租户访问令牌（Tenant Access Token）管理
//! - 应用票据（App Ticket）管理

use crate::api_resp::BaseResponse;
use crate::config::Config;
use serde::{Deserialize, Serialize};

// 租户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenResponse {
    pub tenant_access_token: String,
    pub expire: i32,
}

// 应用访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenResponse {
    pub app_access_token: String,
    pub expire: i32,
}

// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub user_id: String,
    pub open_id: String,
    pub name: String,
    pub en_name: String,
    pub email: String,
    pub mobile: String,
    pub avatar: AvatarInfo,
    pub gender: String,
    pub employee_number: String,
    pub department_ids: Vec<String>,
    pub leader_user_id: String,
    pub position: String,
    pub city: String,
    pub country: String,
    pub work_station: String,
    pub join_time: i64,
    pub custom_attrs: Vec<CustomAttr>,
    pub emails: Vec<EmailInfo>,
    pub mobiles: Vec<MobileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarInfo {
    pub avatar_72: String,
    pub avatar_240: String,
    pub avatar_640: String,
    pub avatar_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttr {
    pub id: String,
    pub name: String,
    pub text: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailInfo {
    pub email: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileInfo {
    pub mobile: String,
    pub r#type: String,
}

// 租户访问令牌构建器（商店应用）
#[derive(Debug, Clone)]
pub struct GetTenantAccessTokenBuilder {
    pub request: GetTenantAccessTokenRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTenantAccessTokenRequest {
    pub app_id: String,
    pub app_secret: String,
}

// 租户访问令牌构建器（自建应用）
#[derive(Debug, Clone)]
pub struct GetTenantAccessTokenInternalBuilder {
    pub request: GetTenantAccessTokenInternalRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTenantAccessTokenInternalRequest {
    pub app_id: String,
    pub app_secret: String,
    pub tenant_key: Option<String>,
}

// 应用访问令牌构建器（商店应用）
#[derive(Debug, Clone)]
pub struct GetAppAccessTokenBuilder {
    pub request: GetAppAccessTokenRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppAccessTokenRequest {
    pub app_id: String,
    pub app_secret: String,
}

// 应用访问令牌构建器（自建应用）
#[derive(Debug, Clone)]
pub struct GetAppAccessTokenInternalBuilder {
    pub request: GetAppAccessTokenInternalRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppAccessTokenInternalRequest {
    pub app_id: String,
    pub app_secret: String,
}

// 用户信息构建器
#[derive(Debug, Clone)]
pub struct GetUserInfoBuilder {
    pub request: GetUserInfoRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub user_access_token: String,
    pub user_id_type: Option<String>,
}

// 应用票据重新发送构建器
#[derive(Debug, Clone)]
pub struct ResendAppTicketBuilder {
    pub request: ResendAppTicketRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendAppTicketRequest {
    pub app_id: String,
    pub app_secret: String,
    pub callback_address: Option<String>,
}

// 服务类型定义
#[derive(Debug, Clone)]
pub struct TenantAccessTokenService {
    pub config: Config,
}

impl TenantAccessTokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone)]
pub struct AppAccessTokenService {
    pub config: Config,
}

impl AppAccessTokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone)]
pub struct UserInfoService {
    pub config: Config,
}

impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone)]
pub struct AppTicketService {
    pub config: Config,
}

impl AppTicketService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 类型别名
pub type UserInfo = UserInfoResponse;

// 请求和响应类型别名
pub type GetTenantAccessTokenResponse = BaseResponse<TenantAccessTokenResponse>;
pub type GetAppAccessTokenResponse = BaseResponse<AppAccessTokenResponse>;
pub type GetUserInfoResponse = BaseResponse<UserInfoResponse>;
pub type ResendAppTicketResponse = BaseResponse<serde_json::Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_creation() {
        let _req = GetTenantAccessTokenRequest {
            app_id: "test".to_string(),
            app_secret: "test".to_string(),
        };

        let _user_info = UserInfoResponse {
            user_id: "test".to_string(),
            open_id: "test".to_string(),
            name: "test".to_string(),
            en_name: "test".to_string(),
            email: "test@example.com".to_string(),
            mobile: "+86138****8888".to_string(),
            avatar: AvatarInfo {
                avatar_72: "http://example.com/72.png".to_string(),
                avatar_240: "http://example.com/240.png".to_string(),
                avatar_640: "http://example.com/640.png".to_string(),
                avatar_origin: "http://example.com/origin.png".to_string(),
            },
            gender: "unknown".to_string(),
            employee_number: "12345".to_string(),
            department_ids: vec!["dept_123".to_string()],
            leader_user_id: "leader_123".to_string(),
            position: "Engineer".to_string(),
            city: "Beijing".to_string(),
            country: "China".to_string(),
            work_station: "Office".to_string(),
            join_time: 1609459200,
            custom_attrs: vec![],
            emails: vec![],
            mobiles: vec![],
        };
    }
}
