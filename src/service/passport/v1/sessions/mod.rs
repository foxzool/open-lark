//! 会话管理API v1
//!
//! 提供用户会话相关的API功能，包括登录、登出、会话查询等。

use crate::{
    config::Config, constants::AccessTokenType, endpoints_original::Endpoints, http::Transport,
};
use open_lark_core::{error::LarkAPIError, ApiRequest, SDKResult};
use crate::service::passport::models::{LogoutRequest, LogoutResponse, PassportResponse};

/// 会话管理服务
#[derive(Debug, Clone)]
pub struct SessionsService {
    config: Config,
}

impl SessionsService {
    /// 创建新的会话管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 退出登录
    ///
    /// 批量退出指定用户的登录会话
    ///
    /// # 参数
    /// * `request` - 退出登录请求，包含用户ID列表和用户ID类型
    ///
    /// # 返回
    /// 返回退出操作的结果，包括成功和失败的用户数量
    ///
    /// # 示例
    /// ```rust
    /// let request = LogoutRequest {
    ///     user_ids: vec!["user_123".to_string()],
    ///     user_id_type: "open_id".to_string(),
    /// };
    /// let response = passport_v1.sessions.logout(&request).await?;
    /// ```
    pub async fn logout(&self, request: &LogoutRequest) -> SDKResult<LogoutResponse> {
        // 验证用户ID列表不为空
        if request.user_ids.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "用户ID列表不能为空".to_string(),
            ));
        }

        let mut api_req = ApiRequest::default();
        api_req.set_api_path(Endpoints::PASSPORT_V1_SESSIONS_LOGOUT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(request)?);

        let resp =
            Transport::<PassportResponse<LogoutResponse>>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.and_then(|r| r.data).unwrap_or_default())
    }

    /// 创建退出登录构建器
    ///
    /// 使用构建器模式创建退出登录请求
    ///
    /// # 示例
    /// ```rust
    /// let response = passport_v1.sessions
    ///     .logout_builder()
    ///     .user_id("user_123")
    ///     .user_id_type("open_id")
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn logout_builder(&self) -> LogoutBuilder {
        LogoutBuilder::new(self.config.clone())
    }
}

/// 退出登录构建器
pub struct LogoutBuilder {
    config: Config,
    user_ids: Vec<String>,
    user_id_type: String,
}

impl LogoutBuilder {
    /// 创建新的退出登录构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            user_id_type: "open_id".to_string(),
        }
    }

    /// 添加单个用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户ID
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids.extend(user_ids);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }

    /// 执行退出登录操作
    pub async fn execute(self) -> SDKResult<LogoutResponse> {
        // 验证用户ID列表不为空
        if self.user_ids.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "用户ID列表不能为空".to_string(),
            ));
        }

        let request = LogoutRequest {
            user_ids: self.user_ids,
            user_id_type: self.user_id_type,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_api_path(Endpoints::PASSPORT_V1_SESSIONS_LOGOUT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let resp =
            Transport::<PassportResponse<LogoutResponse>>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.and_then(|r| r.data).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sessions_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        // Service created successfully
    }

    #[test]
    fn test_logout_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        let builder = service.logout_builder();

        // Builder created successfully
        assert_eq!(builder.user_ids.len(), 0);
        assert_eq!(builder.user_id_type, "open_id");
    }

    #[test]
    fn test_logout_builder_with_user_id() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        let builder = service
            .logout_builder()
            .user_id("user_123")
            .user_id_type("user_id");

        assert_eq!(builder.user_ids.len(), 1);
        assert_eq!(builder.user_ids[0], "user_123");
        assert_eq!(builder.user_id_type, "user_id");
    }

    #[test]
    fn test_logout_builder_with_multiple_users() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        let builder = service
            .logout_builder()
            .user_ids(vec!["user_123".to_string(), "user_456".to_string()]);

        assert_eq!(builder.user_ids.len(), 2);
        assert_eq!(builder.user_ids[0], "user_123");
        assert_eq!(builder.user_ids[1], "user_456");
    }

    #[test]
    fn test_logout_request_serialization() {
        let request = LogoutRequest {
            user_ids: vec!["user_123".to_string(), "user_456".to_string()],
            user_id_type: "open_id".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("user_123"));
        assert!(serialized.contains("user_456"));
        assert!(serialized.contains("open_id"));
    }

    #[tokio::test]
    async fn test_logout_request_empty_user_ids() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        let builder = service.logout_builder();

        // Execute should fail with empty user_ids
        let result = builder.execute().await;
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("用户ID列表不能为空"));
        }
    }

    #[tokio::test]
    async fn test_logout_traditional_method_empty_user_ids() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SessionsService::new(config);
        let request = LogoutRequest {
            user_ids: vec![],
            user_id_type: "open_id".to_string(),
        };

        // Execute should fail with empty user_ids
        let result = service.logout(&request).await;
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("用户ID列表不能为空"));
        }
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            Endpoints::PASSPORT_V1_SESSIONS_LOGOUT,
            "/open-apis/passport/v1/sessions/logout"
        );
    }
}
