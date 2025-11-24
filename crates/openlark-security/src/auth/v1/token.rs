//! 令牌管理API v1
//!
//! 提供访问令牌和刷新令牌的管理功能。

use crate::auth::service::TokenValidationResponse;
use crate::error::{SecurityError, SecurityResult};
use crate::models::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 访问令牌请求构建器
#[derive(Debug, Clone)]
pub struct AccessTokenRequestBuilder {
    request: AccessTokenRequest,
}

impl AccessTokenRequestBuilder {
    /// 创建新的访问令牌请求构建器
    pub fn new() -> Self {
        Self {
            request: AccessTokenRequest {
                grant_type: "authorization_code".to_string(),
                code: String::new(),
                client_id: None,
                client_secret: None,
                redirect_uri: None,
                refresh_token: None,
                scope: None,
                state: None,
                code_verifier: None,
            },
        }
    }

    /// 设置授权码
    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.request.code = code.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.request.client_id = Some(client_id.into());
        self
    }

    /// 设置客户端密钥
    pub fn client_secret<S: Into<String>>(mut self, client_secret: S) -> Self {
        self.request.client_secret = Some(client_secret.into());
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri<S: Into<String>>(mut self, redirect_uri: S) -> Self {
        self.request.redirect_uri = Some(redirect_uri.into());
        self
    }

    /// 设置刷新令牌
    pub fn refresh_token<S: Into<String>>(mut self, refresh_token: S) -> Self {
        self.request.refresh_token = Some(refresh_token.into());
        self.request.grant_type = "refresh_token".to_string();
        self
    }

    /// 设置权限范围
    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    /// 设置状态参数
    pub fn state<S: Into<String>>(mut self, state: S) -> Self {
        self.request.state = Some(state.into());
        self
    }

    /// 设置代码验证器（PKCE）
    pub fn code_verifier<S: Into<String>>(mut self, code_verifier: S) -> Self {
        self.request.code_verifier = Some(code_verifier.into());
        self
    }

    /// 构建访问令牌请求
    pub fn build(self) -> AccessTokenRequest {
        self.request
    }
}

impl Default for AccessTokenRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 令牌刷新请求构建器
#[derive(Debug, Clone)]
pub struct RefreshTokenRequestBuilder {
    request: RefreshTokenRequest,
}

impl RefreshTokenRequestBuilder {
    /// 创建新的令牌刷新请求构建器
    pub fn new() -> Self {
        Self {
            request: RefreshTokenRequest {
                refresh_token: String::new(),
                client_id: None,
                client_secret: None,
                scope: None,
                grant_type: "refresh_token".to_string(),
            },
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token<S: Into<String>>(mut self, refresh_token: S) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.request.client_id = Some(client_id.into());
        self
    }

    /// 设置客户端密钥
    pub fn client_secret<S: Into<String>>(mut self, client_secret: S) -> Self {
        self.request.client_secret = Some(client_secret.into());
        self
    }

    /// 设置权限范围
    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    /// 构建令牌刷新请求
    pub fn build(self) -> RefreshTokenRequest {
        self.request
    }
}

impl Default for RefreshTokenRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取访问令牌API
///
/// # auth_get_access_token_v1
///
/// 通过授权码或刷新令牌获取访问令牌。
///
/// ## 请求参数
///
/// - `grant_type`: 授权类型（authorization_code 或 refresh_token）
/// - `code`: 授权码（授权码流程时必需）
/// - `refresh_token`: 刷新令牌（刷新流程时必需）
/// - `client_id`: 客户端ID
/// - `client_secret`: 客户端密钥
/// - `redirect_uri`: 重定向URI（授权码流程时必需）
/// - `scope`: 权限范围（可选）
/// - `state`: 状态参数（可选）
/// - `code_verifier`: 代码验证器（PKCE，可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// // 授权码流程
/// let request = AccessTokenRequestBuilder::new()
///     .code("authorization_code_here")
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .redirect_uri("https://your-app.com/callback")
///     .scope("contact:user.base:readonly message:message.send")
///     .build();
///
/// let response = auth_get_access_token_v1(request).await?;
///
/// // 刷新令牌流程
/// let refresh_request = AccessTokenRequestBuilder::new()
///     .refresh_token("refresh_token_here")
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .build();
///
/// let refresh_response = auth_get_access_token_v1(refresh_request).await?;
/// ```
///
/// ## 返回值
///
/// 返回 `AccessTokenResponse`，包含访问令牌、刷新令牌和过期时间等信息。
pub async fn auth_get_access_token_v1(
    request: AccessTokenRequest,
) -> SecurityResult<AccessTokenResponse> {
    info!(
        "获取访问令牌: grant_type={}, client_id={:?}",
        request.grant_type,
        request
            .client_id
            .as_ref()
            .map(|id| &id[..std::cmp::min(8, id.len())])
    );

    // 验证请求参数
    validate_access_token_request(&request)?;

    // 根据授权类型处理请求
    let response = match request.grant_type.as_str() {
        "authorization_code" => handle_authorization_code_flow(&request).await?,
        "refresh_token" => handle_refresh_token_flow(&request).await?,
        _ => {
            return Err(SecurityError::InvalidParameter {
                parameter: "grant_type".to_string(),
                reason: "不支持的授权类型".to_string(),
            });
        }
    };

    info!(
        "访问令牌获取成功: token_prefix={}",
        &response.access_token[..std::cmp::min(10, response.access_token.len())]
    );
    Ok(response)
}

/// 刷新访问令牌API
///
/// # auth_refresh_access_token_v1
///
/// 使用刷新令牌获取新的访问令牌。
///
/// ## 请求参数
///
/// - `refresh_token`: 刷新令牌
/// - `client_id`: 客户端ID（可选）
/// - `client_secret`: 客户端密钥（可选）
/// - `scope`: 权限范围（可选）
/// - `grant_type`: 授权类型（固定为 refresh_token）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = RefreshTokenRequestBuilder::new()
///     .refresh_token("your_refresh_token")
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .scope("contact:user.base:readonly")
///     .build();
///
/// let response = auth_refresh_access_token_v1(request).await?;
/// ```
pub async fn auth_refresh_access_token_v1(
    request: RefreshTokenRequest,
) -> SecurityResult<AccessTokenResponse> {
    info!(
        "刷新访问令牌: client_id={:?}",
        request
            .client_id
            .as_ref()
            .map(|id| &id[..std::cmp::min(8, id.len())])
    );

    // 验证刷新令牌
    validate_refresh_token(&request.refresh_token).await?;

    // 验证客户端信息（如果提供）
    if let (Some(client_id), Some(client_secret)) = (&request.client_id, &request.client_secret) {
        validate_client_credentials(client_id, client_secret).await?;
    }

    // 执行令牌刷新
    let response = refresh_access_token_internal(&request).await?;

    info!(
        "访问令牌刷新成功: token_prefix={}",
        &response.access_token[..std::cmp::min(10, response.access_token.len())]
    );
    Ok(response)
}

/// 撤销访问令牌API
///
/// # auth_revoke_access_token_v1
///
/// 撤销指定的访问令牌，使其立即失效。
///
/// ## 请求参数
///
/// - `access_token`: 要撤销的访问令牌
/// - `client_id`: 客户端ID
/// - `client_secret`: 客户端密钥
/// - `revoke_reason`: 撤销原因（可选）
/// - `force`: 是否强制撤销（可选，默认false）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = RevokeTokenRequest {
///     access_token: "access_token_to_revoke".to_string(),
///     client_id: "your_client_id".to_string(),
///     client_secret: "your_client_secret".to_string(),
///     revoke_reason: Some("用户主动登出".to_string()),
///     force: Some(false),
/// };
///
/// let response = auth_revoke_access_token_v1(request).await?;
/// ```
pub async fn auth_revoke_access_token_v1(
    request: RevokeTokenRequest,
) -> SecurityResult<RevokeTokenResponse> {
    info!(
        "撤销访问令牌: token_prefix={}",
        &request.access_token[..std::cmp::min(10, request.access_token.len())]
    );

    // 验证客户端凭据
    validate_client_credentials(&request.client_id, &request.client_secret).await?;

    // 验证访问令牌格式
    validate_access_token_format(&request.access_token)?;

    // 检查令牌是否已经过期
    let token_info = get_token_info(&request.access_token).await?;
    if token_info.is_expired {
        return Ok(RevokeTokenResponse {
            success: true,
            access_token: request.access_token,
            revoked_at: chrono::Utc::now(),
            message: "令牌已过期，无需撤销".to_string(),
        });
    }

    // 执行令牌撤销
    let revoked_at = revoke_token_internal(&request.access_token, &request.revoke_reason).await?;

    info!("访问令牌撤销成功");
    Ok(RevokeTokenResponse {
        success: true,
        access_token: request.access_token,
        revoked_at,
        message: "令牌撤销成功".to_string(),
    })
}

/// 获取令牌信息API
///
/// # auth_get_token_info_v1
///
/// 获取访问令牌的详细信息，包括用户信息、权限范围、过期时间等。
///
/// ## 请求参数
///
/// - `access_token`: 访问令牌
/// - `include_permissions`: 是否包含权限详情（可选，默认false）
/// - `include_session_info`: 是否包含会话信息（可选，默认false）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = TokenInfoRequest {
///     access_token: "your_access_token".to_string(),
///     include_permissions: Some(true),
///     include_session_info: Some(true),
/// };
///
/// let response = auth_get_token_info_v1(request).await?;
/// ```
pub async fn auth_get_token_info_v1(
    request: TokenInfoRequest,
) -> SecurityResult<TokenInfoResponse> {
    info!(
        "获取令牌信息: token_prefix={}",
        &request.access_token[..std::cmp::min(10, request.access_token.len())]
    );

    // 验证访问令牌
    let token_validation = validate_access_token_detailed(&request.access_token).await?;

    if !token_validation.valid {
        return Ok(TokenInfoResponse {
            valid: false,
            access_token: request.access_token,
            token_type: "Bearer".to_string(),
            user_id: None,
            app_id: None,
            tenant_key: None,
            scope: vec![],
            expires_at: token_validation.expires_at,
            issued_at: None,
            permissions: vec![],
            session_info: None,
            error: Some("令牌无效或已过期".to_string()),
        });
    }

    // 获取详细的令牌信息
    let token_info = get_token_detailed_info(&request.access_token, &request).await?;

    info!(
        "令牌信息获取成功: user_id={:?}, expires_at={:?}",
        token_info.user_id, token_info.expires_at
    );
    Ok(token_info)
}

// ============ 辅助函数 ============

/// 验证访问令牌请求
fn validate_access_token_request(request: &AccessTokenRequest) -> SecurityResult<()> {
    match request.grant_type.as_str() {
        "authorization_code" => {
            if request.code.is_empty() {
                return Err(SecurityError::MissingParameter {
                    parameter: "code".to_string(),
                    message: "授权码流程需要code参数".to_string(),
                });
            }
            if request.redirect_uri.is_none() {
                return Err(SecurityError::MissingParameter {
                    parameter: "redirect_uri".to_string(),
                    message: "授权码流程需要redirect_uri参数".to_string(),
                });
            }
        }
        "refresh_token" => {
            if request.refresh_token.is_none() {
                return Err(SecurityError::MissingParameter {
                    parameter: "refresh_token".to_string(),
                    message: "刷新流程需要refresh_token参数".to_string(),
                });
            }
        }
        _ => {
            return Err(SecurityError::InvalidParameter {
                parameter: "grant_type".to_string(),
                reason: "不支持的授权类型".to_string(),
            });
        }
    }
    Ok(())
}

/// 处理授权码流程
async fn handle_authorization_code_flow(
    request: &AccessTokenRequest,
) -> SecurityResult<AccessTokenResponse> {
    // 验证授权码
    let auth_code_info = validate_authorization_code(&request.code).await?;

    // 验证重定向URI
    if let Some(redirect_uri) = &request.redirect_uri {
        if auth_code_info.redirect_uri != *redirect_uri {
            return Err(SecurityError::InvalidParameter {
                parameter: "redirect_uri".to_string(),
                reason: "重定向URI不匹配".to_string(),
            });
        }
    }

    // 验证PKCE（如果使用）
    if let (Some(code_challenge), Some(code_verifier)) =
        (&auth_code_info.code_challenge, &request.code_verifier)
    {
        if !verify_pkce(code_challenge, code_verifier) {
            return Err(SecurityError::AuthenticationError {
                message: "PKCE验证失败".to_string(),
                error_code: Some("PKCE_VERIFICATION_FAILED".to_string()),
            });
        }
    }

    // 生成访问令牌和刷新令牌
    let now = chrono::Utc::now();
    let access_token =
        generate_access_token(&auth_code_info.user_id, &auth_code_info.app_id).await?;
    let refresh_token =
        generate_refresh_token(&auth_code_info.user_id, &auth_code_info.app_id).await?;

    // 保存令牌信息
    save_token_info(
        &access_token,
        &refresh_token,
        &auth_code_info,
        &request.scope,
    )
    .await?;

    Ok(AccessTokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 7200, // 2小时
        refresh_token: Some(refresh_token),
        scope: request.scope.clone(),
        user_id: Some(auth_code_info.user_id),
        app_id: Some(auth_code_info.app_id),
        tenant_key: Some(auth_code_info.tenant_key),
        issued_at: Some(now),
        expires_at: Some(now + Duration::seconds(7200)),
    })
}

/// 处理刷新令牌流程
async fn handle_refresh_token_flow(
    request: &AccessTokenRequest,
) -> SecurityResult<AccessTokenResponse> {
    let refresh_token =
        request
            .refresh_token
            .as_ref()
            .ok_or_else(|| SecurityError::MissingParameter {
                parameter: "refresh_token".to_string(),
                message: "刷新流程需要refresh_token参数".to_string(),
            })?;

    // 验证刷新令牌
    let refresh_token_info = validate_refresh_token_internal(refresh_token).await?;

    // 生成新的访问令牌
    let now = chrono::Utc::now();
    let new_access_token =
        generate_access_token(&refresh_token_info.user_id, &refresh_token_info.app_id).await?;
    let new_refresh_token =
        generate_refresh_token(&refresh_token_info.user_id, &refresh_token_info.app_id).await?;

    // 更新令牌信息
    update_token_info(
        &new_access_token,
        &new_refresh_token,
        &refresh_token_info,
        &request.scope,
    )
    .await?;

    Ok(AccessTokenResponse {
        access_token: new_access_token,
        token_type: "Bearer".to_string(),
        expires_in: 7200,
        refresh_token: Some(new_refresh_token),
        scope: request.scope.clone().or(refresh_token_info.scope),
        user_id: Some(refresh_token_info.user_id),
        app_id: Some(refresh_token_info.app_id),
        tenant_key: Some(refresh_token_info.tenant_key),
        issued_at: Some(now),
        expires_at: Some(now + Duration::seconds(7200)),
    })
}

/// 验证刷新令牌
async fn validate_refresh_token(refresh_token: &str) -> SecurityResult<()> {
    if refresh_token.is_empty() {
        return Err(SecurityError::AuthenticationError {
            message: "刷新令牌不能为空".to_string(),
            error_code: Some("MISSING_REFRESH_TOKEN".to_string()),
        });
    }

    if !refresh_token.starts_with("refresh_token_") {
        return Err(SecurityError::AuthenticationError {
            message: "无效的刷新令牌格式".to_string(),
            error_code: Some("INVALID_REFRESH_TOKEN_FORMAT".to_string()),
        });
    }

    // 检查刷新令牌是否在黑名单中
    if is_refresh_token_blacklisted(refresh_token).await? {
        return Err(SecurityError::AuthenticationError {
            message: "刷新令牌已被撤销".to_string(),
            error_code: Some("REFRESH_TOKEN_REVOKED".to_string()),
        });
    }

    Ok(())
}

/// 验证客户端凭据
async fn validate_client_credentials(client_id: &str, client_secret: &str) -> SecurityResult<()> {
    if client_id.is_empty() || client_secret.is_empty() {
        return Err(SecurityError::AuthenticationError {
            message: "客户端ID和密钥不能为空".to_string(),
            error_code: Some("MISSING_CLIENT_CREDENTIALS".to_string()),
        });
    }

    // 在实际实现中，应该验证客户端凭据的有效性
    // 这里模拟验证逻辑
    if !client_id.starts_with("client_") || !client_secret.starts_with("secret_") {
        return Err(SecurityError::AuthenticationError {
            message: "无效的客户端凭据".to_string(),
            error_code: Some("INVALID_CLIENT_CREDENTIALS".to_string()),
        });
    }

    Ok(())
}

/// 验证访问令牌格式
fn validate_access_token_format(access_token: &str) -> SecurityResult<()> {
    if access_token.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "access_token".to_string(),
            reason: "访问令牌不能为空".to_string(),
        });
    }

    if !access_token.starts_with("access_token_") {
        return Err(SecurityError::InvalidParameter {
            parameter: "access_token".to_string(),
            reason: "无效的访问令牌格式".to_string(),
        });
    }

    Ok(())
}

// ============ 模拟实现函数 ============

/// 验证授权码
async fn validate_authorization_code(code: &str) -> SecurityResult<AuthCodeInfo> {
    // 模拟授权码验证
    Ok(AuthCodeInfo {
        user_id: "user_12345".to_string(),
        app_id: "app_67890".to_string(),
        tenant_key: "tenant_abc".to_string(),
        redirect_uri: "https://your-app.com/callback".to_string(),
        code_challenge: None,
        expires_at: chrono::Utc::now() + Duration::minutes(10),
    })
}

/// 验证刷新令牌（内部）
async fn validate_refresh_token_internal(refresh_token: &str) -> SecurityResult<RefreshTokenInfo> {
    // 模拟刷新令牌验证
    Ok(RefreshTokenInfo {
        user_id: "user_12345".to_string(),
        app_id: "app_67890".to_string(),
        tenant_key: "tenant_abc".to_string(),
        scope: Some("contact:user.base:readonly message:message.send".to_string()),
        created_at: chrono::Utc::now() - Duration::days(7),
        expires_at: chrono::Utc::now() + Duration::days(23),
    })
}

/// 获取令牌信息
async fn get_token_info(access_token: &str) -> SecurityResult<TokenInfo> {
    // 模拟获取令牌信息
    let now = chrono::Utc::now();
    let is_expired = !access_token.ends_with("_valid");

    Ok(TokenInfo {
        is_expired,
        expires_at: Some(now + Duration::hours(2)),
    })
}

/// 详细验证访问令牌
async fn validate_access_token_detailed(
    access_token: &str,
) -> SecurityResult<TokenValidationResponse> {
    validate_access_token_format(access_token)?;

    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(2);

    Ok(TokenValidationResponse {
        valid: access_token.ends_with("_valid"),
        user_id: Some("user_12345".to_string()),
        expires_at: Some(expires_at),
        scopes: vec![
            "contact:user.base:readonly".to_string(),
            "message:message.send".to_string(),
        ],
        error: None,
    })
}

/// 生成访问令牌
async fn generate_access_token(user_id: &str, app_id: &str) -> SecurityResult<String> {
    let token = format!(
        "access_token_{}_{}_{}_{}",
        user_id,
        app_id,
        uuid::Uuid::new_v4(),
        "valid"
    );
    Ok(token)
}

/// 生成刷新令牌
async fn generate_refresh_token(user_id: &str, app_id: &str) -> SecurityResult<String> {
    let token = format!(
        "refresh_token_{}_{}_{}",
        user_id,
        app_id,
        uuid::Uuid::new_v4()
    );
    Ok(token)
}

/// 保存令牌信息
async fn save_token_info(
    access_token: &str,
    refresh_token: &str,
    auth_code_info: &AuthCodeInfo,
    scope: &Option<String>,
) -> SecurityResult<()> {
    info!(
        "保存令牌信息: access_token_prefix={}, refresh_token_prefix={}",
        &access_token[..std::cmp::min(20, access_token.len())],
        &refresh_token[..std::cmp::min(20, refresh_token.len())]
    );
    // 在实际实现中，应该将令牌信息保存到数据库或缓存
    Ok(())
}

/// 更新令牌信息
async fn update_token_info(
    access_token: &str,
    refresh_token: &str,
    refresh_token_info: &RefreshTokenInfo,
    scope: &Option<String>,
) -> SecurityResult<()> {
    info!(
        "更新令牌信息: access_token_prefix={}, refresh_token_prefix={}",
        &access_token[..std::cmp::min(20, access_token.len())],
        &refresh_token[..std::cmp::min(20, refresh_token.len())]
    );
    // 在实际实现中，应该更新数据库中的令牌信息
    Ok(())
}

/// 刷新访问令牌（内部）
async fn refresh_access_token_internal(
    request: &RefreshTokenRequest,
) -> SecurityResult<AccessTokenResponse> {
    let refresh_token_info = validate_refresh_token_internal(&request.refresh_token).await?;

    let now = chrono::Utc::now();
    let access_token =
        generate_access_token(&refresh_token_info.user_id, &refresh_token_info.app_id).await?;
    let new_refresh_token =
        generate_refresh_token(&refresh_token_info.user_id, &refresh_token_info.app_id).await?;

    Ok(AccessTokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 7200,
        refresh_token: Some(new_refresh_token),
        scope: request.scope.clone().or(refresh_token_info.scope),
        user_id: Some(refresh_token_info.user_id),
        app_id: Some(refresh_token_info.app_id),
        tenant_key: Some(refresh_token_info.tenant_key),
        issued_at: Some(now),
        expires_at: Some(now + Duration::seconds(7200)),
    })
}

/// 撤销令牌（内部）
async fn revoke_token_internal(
    access_token: &str,
    reason: &Option<String>,
) -> SecurityResult<DateTime<Utc>> {
    info!(
        "撤销令牌: access_token_prefix={}, reason={:?}",
        &access_token[..std::cmp::min(10, access_token.len())],
        reason
    );
    // 在实际实现中，应该将令牌加入黑名单并从数据库中删除
    Ok(chrono::Utc::now())
}

/// 获取详细令牌信息
async fn get_token_detailed_info(
    access_token: &str,
    request: &TokenInfoRequest,
) -> SecurityResult<TokenInfoResponse> {
    let token_validation = validate_access_token_detailed(access_token).await?;

    let permissions = if request.include_permissions.unwrap_or(false) {
        vec![
            Permission {
                scope: "contact:user.base:readonly".to_string(),
                granted: true,
                expires_at: Some(chrono::Utc::now() + Duration::hours(2)),
            },
            Permission {
                scope: "message:message.send".to_string(),
                granted: true,
                expires_at: Some(chrono::Utc::now() + Duration::hours(2)),
            },
        ]
    } else {
        vec![]
    };

    let session_info = if request.include_session_info.unwrap_or(false) {
        Some(SessionInfo {
            session_id: Some(format!("session_{}", uuid::Uuid::new_v4())),
            device_id: Some("device_001".to_string()),
            ip_address: Some("192.168.1.100".to_string()),
            user_agent: Some(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            ),
            created_at: chrono::Utc::now() - Duration::minutes(30),
            last_activity: chrono::Utc::now() - Duration::minutes(5),
        })
    } else {
        None
    };

    Ok(TokenInfoResponse {
        valid: token_validation.valid,
        access_token: access_token.to_string(),
        token_type: "Bearer".to_string(),
        user_id: token_validation.user_id,
        app_id: Some("app_67890".to_string()),
        tenant_key: Some("tenant_abc".to_string()),
        scope: token_validation.scopes,
        expires_at: token_validation.expires_at,
        issued_at: Some(chrono::Utc::now() - Duration::minutes(30)),
        permissions,
        session_info,
        error: token_validation.error,
    })
}

/// 验证PKCE
fn verify_pkce(code_challenge: &str, code_verifier: &str) -> bool {
    // 在实际实现中，应该使用SHA256计算code_challenge并验证
    // 这里简化为模拟验证
    !code_challenge.is_empty() && !code_verifier.is_empty()
}

/// 检查刷新令牌是否在黑名单中
async fn is_refresh_token_blacklisted(refresh_token: &str) -> SecurityResult<bool> {
    // 模拟黑名单检查
    let blacklisted_tokens = vec!["blacklisted_refresh_token_1".to_string()];
    Ok(blacklisted_tokens.contains(&refresh_token.to_string()))
}

// ============ 请求和响应模型定义 ============

/// 访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenRequest {
    /// 授权类型
    pub grant_type: String,
    /// 授权码
    pub code: String,
    /// 客户端ID
    pub client_id: Option<String>,
    /// 客户端密钥
    pub client_secret: Option<String>,
    /// 重定向URI
    pub redirect_uri: Option<String>,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 权限范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
    /// 代码验证器（PKCE）
    pub code_verifier: Option<String>,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
    /// 客户端ID
    pub client_id: Option<String>,
    /// 客户端密钥
    pub client_secret: Option<String>,
    /// 权限范围
    pub scope: Option<String>,
    /// 授权类型
    pub grant_type: String,
}

/// 撤销令牌请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeTokenRequest {
    /// 访问令牌
    pub access_token: String,
    /// 客户端ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 撤销原因
    pub revoke_reason: Option<String>,
    /// 是否强制撤销
    pub force: Option<bool>,
}

/// 令牌信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfoRequest {
    /// 访问令牌
    pub access_token: String,
    /// 是否包含权限详情
    pub include_permissions: Option<bool>,
    /// 是否包含会话信息
    pub include_session_info: Option<bool>,
}

/// 访问令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 权限范围
    pub scope: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 租户key
    pub tenant_key: Option<String>,
    /// 签发时间
    pub issued_at: Option<DateTime<Utc>>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
}

/// 撤销令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeTokenResponse {
    /// 是否成功
    pub success: bool,
    /// 访问令牌
    pub access_token: String,
    /// 撤销时间
    pub revoked_at: DateTime<Utc>,
    /// 消息
    pub message: String,
}

/// 令牌信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfoResponse {
    /// 是否有效
    pub valid: bool,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 用户ID
    pub user_id: Option<String>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 租户key
    pub tenant_key: Option<String>,
    /// 权限范围
    pub scope: Vec<String>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 签发时间
    pub issued_at: Option<DateTime<Utc>>,
    /// 权限详情
    pub permissions: Vec<Permission>,
    /// 会话信息
    pub session_info: Option<SessionInfo>,
    /// 错误信息
    pub error: Option<String>,
}

/// 权限信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    /// 权限范围
    pub scope: String,
    /// 是否已授权
    pub granted: bool,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
}

/// 会话信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 会话ID
    pub session_id: Option<String>,
    /// 设备ID
    pub device_id: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后活动时间
    pub last_activity: DateTime<Utc>,
}

// ============ 内部数据结构 ============

/// 授权码信息
#[derive(Debug)]
struct AuthCodeInfo {
    user_id: String,
    app_id: String,
    tenant_key: String,
    redirect_uri: String,
    code_challenge: Option<String>,
    expires_at: DateTime<Utc>,
}

/// 刷新令牌信息
#[derive(Debug)]
struct RefreshTokenInfo {
    user_id: String,
    app_id: String,
    tenant_key: String,
    scope: Option<String>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

/// 令牌信息
#[derive(Debug)]
struct TokenInfo {
    is_expired: bool,
    expires_at: Option<DateTime<Utc>>,
}
