//! 获取登录用户信息API
//!
//! 通过 user_access_token 获取登录用户的信息。
//!
//! API文档: https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/get

use crate::models::{UserInfoResponse, AuthConfig};
use openlark_core::SDKResult;
use crate::utils::authen_auth_config_to_core;
use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 用户信息请求
/// 此API不需要请求体，只需要user_access_token作为授权
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoRequest {}

/// 用户信息构建器
pub struct UserInfoBuilder {
    config: Config,
    user_access_token: Option<String>,
}

impl UserInfoBuilder {
    /// 创建新的用户信息构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_access_token: None,
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, token: impl Into<String>) -> Self {
        self.user_access_token = Some(token.into());
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> SDKResult<UserInfoResponse> {
        // 验证必要参数
        if self.user_access_token.is_none() || self.user_access_token.as_ref().unwrap().is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "user_access_token",
                "用户访问令牌不能为空".to_string(),
                self.user_access_token.as_deref().map(|s| s.to_string()),
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);
        let api_request = ApiRequest::<UserInfoResponse>::get(&url);

        // 设置用户访问令牌
        let request_option = RequestOption::builder()
            .user_access_token(self.user_access_token.unwrap())
            .build();

        // 使用 openlark-core 的传输层发送请求
        let response = Transport::request(api_request, &self.config, Some(request_option)).await?;

        // 检查响应状态
        if response.code() != 0 {
            return Err(crate::error::map_feishu_auth_error(
                response.code(),
                response.raw().msg.as_str(),
                response.raw().request_id.as_deref(),
            ));
        }

        // 构建响应对象
        let user_info = response.data.unwrap();
        Ok(user_info)
    }
}

/// 用户信息服务
pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(authen_auth_config_to_core(auth_config))
    }

    /// 获取用户信息（需要设置user_access_token）
    pub fn get(&self) -> UserInfoBuilder {
        UserInfoBuilder::new(self.config.clone())
    }

    /// 使用指定的用户访问令牌获取用户信息
    pub fn with_user_access_token(&self, token: impl Into<String>) -> UserInfoBuilder {
        UserInfoBuilder::new(self.config.clone()).user_access_token(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_request_creation() {
        let request = UserInfoRequest {};
        // 验证请求结构体可以正常创建
        assert!(true);
    }

    #[test]
    fn test_user_info_builder_token_setting() {
        let config = Config::builder().build();
        let builder = UserInfoBuilder::new(config)
            .user_access_token("test_token");

        assert_eq!(builder.user_access_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_user_info_service_creation() {
        let config = Config::builder().build();
        let service = UserInfoService::new(config);

        let builder = service.get();
        // 验证服务可以正常创建
        assert!(true);
    }

    #[test]
    fn test_user_info_service_with_token() {
        let config = Config::builder().build();
        let service = UserInfoService::new(config);

        let builder = service.with_user_access_token("test_token");
        assert_eq!(builder.user_access_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_user_info_service_from_auth_config() {
        let auth_config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = UserInfoService::from_auth_config(auth_config);

        // 验证转换成功
        let builder = service.get();
        assert_eq!(builder.config.base_url, "https://open.feishu.cn");
    }
}