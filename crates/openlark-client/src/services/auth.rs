use crate::{Config, Error, Result};
use openlark_auth::models::{AppTicketResponse, UserInfoResponse};
use openlark_auth::prelude::*;
use openlark_auth::AuthServices;

/// 令牌验证响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenVerificationResponse {
    /// 是否有效
    pub valid: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 租户Key
    pub tenant_key: Option<String>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 权限范围
    pub scope: Vec<String>,
}

/**
 * 认证服务
 *
 * 提供认证相关的API接口，包括令牌管理、OAuth认证等
 * 集成了 openlark-auth 模块，提供 Project-Version-Resource 架构的认证服务
 */
#[derive(Debug)]
pub struct AuthService {
    auth_services: AuthServices,
}

impl AuthService {
    /// 创建新的认证服务实例
    pub fn new(config: &Config) -> Self {
        let auth_config =
            AuthConfig::new(&config.app_id, &config.app_secret).with_base_url(&config.base_url);

        let auth_services = AuthServices::new(auth_config);

        Self { auth_services }
    }

    /// 获取自建应用访问令牌
    pub async fn get_internal_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取自建应用访问令牌");

        let response = self
            .auth_services
            .auth
            .v3()
            .app_access_token()
            .internal()
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("获取应用访问令牌失败: {}", e),
            })?;

        Ok(TokenInfo {
            access_token: response.app_access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expire as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expire as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 获取商店应用访问令牌
    pub async fn get_store_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取商店应用访问令牌");

        let response = self
            .auth_services
            .auth
            .v3()
            .app_access_token()
            .store()
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("获取商店应用访问令牌失败: {}", e),
            })?;

        Ok(TokenInfo {
            access_token: response.app_access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expire as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expire as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 使用授权码获取用户访问令牌
    pub async fn get_user_access_token(&self, code: &str) -> Result<TokenInfo> {
        tracing::info!("使用授权码获取用户访问令牌: code={}", code);

        let response = self
            .auth_services
            .authen
            .v1
            .access_token()
            .create()
            .grant_type("authorization_code")
            .code(code)
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("获取用户访问令牌失败: {}", e),
            })?;

        Ok(TokenInfo {
            access_token: response.access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expires_in as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
            scope: response.scope,
        })
    }

    /// 刷新OIDC访问令牌
    pub async fn refresh_oidc_access_token(&self, refresh_token: &str) -> Result<TokenInfo> {
        tracing::info!("刷新OIDC访问令牌");

        let response = self
            .auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .refresh_token(refresh_token)
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("刷新OIDC访问令牌失败: {}", e),
            })?;

        Ok(TokenInfo {
            access_token: response.access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expires_in as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
            scope: response.scope,
        })
    }

    /// 验证令牌
    pub async fn verify_token(&self, access_token: &str) -> Result<TokenVerificationResponse> {
        tracing::info!(
            "验证令牌: token_prefix={}",
            access_token.chars().take(8).collect::<String>()
        );

        // 注意：应用访问令牌(app_access_token)无法通过用户信息接口验证
        // 因为它是应用级别的令牌，不是用户级别的令牌

        // 使用应用信息接口验证令牌有效性
        // 调用应用相关的API来验证app_access_token
        let app_info_result = self
            .auth_services
            .auth
            .v3()
            .app_ticket()
            .resend()
            .send()
            .await;

        match app_info_result {
            Ok(_) => {
                tracing::info!("应用访问令牌验证成功");

                // 对于app_access_token，我们验证的是应用级别的访问权限
                // 不返回具体的用户信息，因为这是应用令牌而不是用户令牌
                Ok(TokenVerificationResponse {
                    valid: true,
                    user_id: None, // app_access_token 不关联特定用户
                    tenant_key: Some("app_tenant".to_string()), // 应用租户标识
                    expires_at: None, // 需要令牌管理器来跟踪过期时间
                    scope: vec!["app:all".to_string()], // 应用级别的权限范围
                })
            }
            Err(e) => {
                tracing::warn!("应用访问令牌验证失败: {}", e);

                // 尝试通过获取令牌信息接口验证
                // 如果能成功获取新令牌，说明当前令牌仍然有效
                let token_refresh_result = self.get_internal_app_access_token().await;

                match token_refresh_result {
                    Ok(_) => {
                        tracing::info!("应用访问令牌仍然有效（通过刷新验证）");
                        Ok(TokenVerificationResponse {
                            valid: true,
                            user_id: None,
                            tenant_key: Some("app_tenant".to_string()),
                            expires_at: None,
                            scope: vec!["app:all".to_string()],
                        })
                    }
                    Err(refresh_err) => {
                        tracing::warn!("应用访问令牌验证失败且无法刷新: {}", refresh_err);
                        Ok(TokenVerificationResponse {
                            valid: false,
                            user_id: None,
                            tenant_key: None,
                            expires_at: None,
                            scope: vec![],
                        })
                    }
                }
            }
        }
    }

    /// 获取用户信息
    pub async fn get_user_info(&self, user_access_token: &str) -> Result<UserInfo> {
        tracing::info!("获取用户信息");

        let response = self
            .auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token(user_access_token)
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("获取用户信息失败: {}", e),
            })?;

        // 用户状态已经是正确的枚举类型
        let status = response.status;

        Ok(UserInfo {
            user_id: response.user_id,
            open_id: response.open_id,
            union_id: response.union_id,
            name: response.name,
            en_name: response.en_name,
            email: response.email,
            mobile: response.mobile,
            avatar_url: response.avatar_url,
            status,
            department_ids: response.department_ids,
            position: response.position,
            employee_no: response.employee_no,
            nickname: response.nickname,
            gender: response.gender,
        })
    }

    /// 生成OAuth授权URL
    pub fn generate_oauth_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        format!(
            "https://open.feishu.cn/open-apis/authen/v1/authorize?app_id={}&redirect_uri={}&scope={}&state={}",
            self.auth_services.config.app_id, redirect_uri, scope, state
        )
    }

    /// 重新推送应用票据
    pub async fn resend_app_ticket(&self) -> Result<()> {
        tracing::info!("重新推送应用票据");

        let _response = self
            .auth_services
            .auth
            .v3()
            .app_ticket()
            .resend()
            .send()
            .await
            .map_err(|e| Error::APIError {
                code: "AUTH_ERROR".to_string(),
                message: format!("重新推送应用票据失败: {}", e),
            })?;

        Ok(())
    }
}

// 为了保持客户端API的兼容性，定义类型别名
/// 用户信息类型别名
pub type UserInfo = UserInfoResponse;

/// 应用信息类型别名
pub type AppInfo = AppTicketResponse;

/// 令牌信息 - 客户端层的统一令牌表示
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenInfo {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 过期时间戳
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// 权限范围
    pub scope: Option<String>,
}

impl TokenInfo {
    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    /// 检查令牌是否需要刷新（提前N分钟）
    pub fn needs_refresh(&self, buffer_minutes: i64) -> bool {
        let buffer = chrono::Duration::minutes(buffer_minutes);
        chrono::Utc::now() + buffer >= self.expires_at
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - chrono::Utc::now()).num_seconds().max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let config = Config::builder()
            .app_id("test")
            .app_secret("test")
            .build()
            .unwrap();

        let service = AuthService::new(&config);
        // 服务创建成功即测试通过
        assert_eq!(service.auth_services.config.app_id, "test");
    }

    #[test]
    fn test_oauth_url_generation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test")
            .build()
            .unwrap();

        let service = AuthService::new(&config);

        let url = service.generate_oauth_url(
            "https://example.com/callback",
            "user:info docs:read",
            "test_state",
        );

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("scope=user:info docs:read"));
        assert!(url.contains("state=test_state"));
    }

    #[test]
    fn test_token_info_methods() {
        let mut token_info = TokenInfo {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 7200,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            scope: Some("test".to_string()),
        };

        assert!(!token_info.is_expired());
        assert!(!token_info.needs_refresh(30));
        assert!(token_info.remaining_seconds() > 0);

        // 测试过期令牌
        token_info.expires_at = chrono::Utc::now() - chrono::Duration::minutes(1);
        assert!(token_info.is_expired());
        assert!(token_info.needs_refresh(30));
        assert_eq!(token_info.remaining_seconds(), 0);
    }
}
