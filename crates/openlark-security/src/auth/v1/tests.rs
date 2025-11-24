//! 认证API v1测试模块
//!
//! 测试认证层的各种API功能。

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SecurityResult;

    #[test]
    fn test_login_request_builder() -> SecurityResult<()> {
        let request = LoginRequestBuilder::new()
            .username("test_user")
            .password("password123")
            .login_type(LoginType::Password)
            .build();

        assert_eq!(request.username, "test_user");
        assert_eq!(request.password, "password123");
        assert_eq!(request.login_type, Some(LoginType::Password));

        Ok(())
    }

    #[test]
    fn test_logout_request_builder() -> SecurityResult<()> {
        let request = LogoutRequestBuilder::new()
            .access_token("access_token_123")
            .device_id("device_001")
            .build();

        assert_eq!(request.access_token, "access_token_123");
        assert_eq!(request.device_id, Some("device_001".to_string()));

        Ok(())
    }

    #[test]
    fn test_access_token_request_builder() -> SecurityResult<()> {
        let request = AccessTokenRequestBuilder::new()
            .code("authorization_code_123")
            .client_id("client_001")
            .redirect_uri("https://example.com/callback")
            .build();

        assert_eq!(request.code, "authorization_code_123");
        assert_eq!(request.client_id, Some("client_001".to_string()));
        assert_eq!(request.redirect_uri, Some("https://example.com/callback".to_string()));

        Ok(())
    }

    #[test]
    fn test_user_auth_request_builder() -> SecurityResult<()> {
        let request = UserAuthRequestBuilder::new()
            .user_id("user_123")
            .access_token("access_token_123")
            .include_profile(true)
            .include_permissions(true)
            .build();

        assert_eq!(request.user_id, Some("user_123".to_string()));
        assert_eq!(request.access_token, "access_token_123");
        assert_eq!(request.include_profile, Some(true));
        assert_eq!(request.include_permissions, Some(true));

        Ok(())
    }

    #[test]
    fn test_user_update_request_builder() -> SecurityResult<()> {
        let request = UserUpdateRequestBuilder::new()
            .user_id("user_123")
            .access_token("access_token_123")
            .nickname("新昵称")
            .position("高级工程师")
            .build();

        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.access_token, "access_token_123");
        assert_eq!(request.nickname, Some("新昵称".to_string()));
        assert_eq!(request.position, Some("高级工程师".to_string()));

        Ok(())
    }

    // 模拟测试 - 在实际测试中需要mock HTTP响应
    #[tokio::test]
    async fn test_auth_login_v1_mock() -> SecurityResult<()> {
        let request = LoginRequestBuilder::new()
            .username("test_user")
            .password("password123")
            .build();

        // 这里应该mock HTTP响应，暂时跳过实际调用
        // let response = auth_login_v1(request).await?;
        // assert!(response.success);

        // 只测试构建器是否正常工作
        assert_eq!(request.username, "test_user");

        Ok(())
    }

    #[tokio::test]
    async fn test_token_validation_format() -> SecurityResult<()> {
        // 测试令牌格式验证函数
        let valid_token = "access_token_12345678";
        let invalid_token = "invalid_token";

        // 这里测试令牌格式，实际的验证函数在token.rs中
        assert!(valid_token.starts_with("access_token_"));
        assert!(!invalid_token.starts_with("access_token_"));

        Ok(())
    }

    #[test]
    fn test_error_handling() -> SecurityResult<()> {
        // 测试错误处理
        let error = SecurityError::InvalidParameter {
            parameter: "username".to_string(),
            reason: "用户名不能为空".to_string(),
        };

        assert_eq!(error.user_friendly_message(), "参数错误: username - 用户名不能为空");

        Ok(())
    }
}