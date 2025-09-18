use crate::{
    core::{
        error::LarkAPIError,
        validation::{validate_and_sanitize_password, ValidateBuilder, ValidationResult},
        SDKResult,
    },
    service::admin::models::PasswordResetRequest,
};

/// 密码重置请求构建器
#[derive(Debug, Clone, Default)]
pub struct PasswordResetRequestBuilder {
    request: PasswordResetRequest,
}

impl PasswordResetRequestBuilder {
    /// 创建新的密码重置请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl ToString) -> Self {
        self.request.user_id = user_id.to_string();
        self
    }

    /// 设置新密码
    pub fn password(mut self, password: impl ToString) -> Self {
        self.request.password = password.to_string();
        self
    }

    /// 构建密码重置请求
    pub fn build(self) -> SDKResult<PasswordResetRequest> {
        // 验证必填字段
        if self.request.user_id.is_empty() {
            return Err(LarkAPIError::illegal_param(
                "user_id is required".to_string(),
            ));
        }

        // 验证密码强度
        let (sanitized_password, validation_result) =
            validate_and_sanitize_password(self.request.password, "password");

        match validation_result {
            ValidationResult::Valid => {
                // 密码验证通过
                Ok(PasswordResetRequest {
                    user_id: self.request.user_id,
                    password: sanitized_password,
                })
            }
            ValidationResult::Warning(msg) => {
                // 密码有警告但仍可使用
                log::warn!("Password validation warning: {}", msg);
                Ok(PasswordResetRequest {
                    user_id: self.request.user_id,
                    password: sanitized_password,
                })
            }
            ValidationResult::Invalid(msg) => {
                // 密码验证失败
                Err(LarkAPIError::illegal_param(format!(
                    "Invalid password: {}",
                    msg
                )))
            }
        }
    }
}

impl ValidateBuilder for PasswordResetRequestBuilder {
    fn validate(&self) -> ValidationResult {
        // 验证用户ID
        if self.request.user_id.is_empty() {
            return ValidationResult::Invalid("user_id is required".to_string());
        }

        // 验证密码
        validate_and_sanitize_password(self.request.password.clone(), "password").1
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_password_reset_request_builder_valid() {
        let builder = PasswordResetRequestBuilder::new()
            .user_id("test_user_id")
            .password("SecurePass123!");

        let result = builder.build();
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.user_id, "test_user_id");
        assert_eq!(request.password, "SecurePass123!");
    }

    #[test]
    fn test_password_reset_request_builder_invalid_password() {
        let builder = PasswordResetRequestBuilder::new()
            .user_id("test_user_id")
            .password("weak"); // 太短且缺少必要字符

        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_password_reset_request_builder_missing_user_id() {
        let builder = PasswordResetRequestBuilder::new().password("SecurePass123!");

        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_password_sanitization() {
        let builder = PasswordResetRequestBuilder::new()
            .user_id("test_user_id")
            .password("  SecurePass123!  "); // 带空格的密码

        let result = builder.build();
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.password, "SecurePass123!"); // 空格被去除
    }

    #[test]
    fn test_validate_builder_trait() {
        let builder = PasswordResetRequestBuilder::new()
            .user_id("test_user_id")
            .password("SecurePass123!");

        assert!(builder.validate().is_valid());

        let invalid_builder = PasswordResetRequestBuilder::new()
            .user_id("test_user_id")
            .password("weak");

        assert!(!invalid_builder.validate().is_valid());
    }
}
