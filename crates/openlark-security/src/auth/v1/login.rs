//! 登录认证API v1
//!
//! 提供用户登录相关的API接口实现。

use crate::auth::service::{ClientInfo, LoginResponse as ServiceLoginResponse, LoginType};
use crate::error::{SecurityError, SecurityResult};
use crate::models::{TokenType, UserType};
use crate::service::{UserInfo, UserStatus};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// 默认国家代码（中国）
fn default_country_code() -> String {
    "CN".to_string()
}

/// 用户登录请求构建器
#[derive(Debug, Clone)]
pub struct LoginRequestBuilder {
    request: LoginRequest,
}

impl LoginRequestBuilder {
    /// 创建新的登录请求构建器
    pub fn new() -> Self {
        Self {
            request: LoginRequest {
                grant_type: GrantType::AuthorizationCode,
                client_id: String::new(),
                client_secret: String::new(),
                code: None,
                redirect_uri: None,
                refresh_token: None,
                scope: None,
                app_id: None,
                app_secret: None,
                login_type: None,
                captcha: None,
                client_info: None,
                remember_me: None,
            },
        }
    }

    /// 设置客户端ID
    pub fn client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.request.client_id = client_id.into();
        self
    }

    /// 设置客户端密钥
    pub fn client_secret<S: Into<String>>(mut self, client_secret: S) -> Self {
        self.request.client_secret = client_secret.into();
        self
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.request.app_id = Some(app_id.into());
        self
    }

    /// 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.request.app_secret = Some(app_secret.into());
        self
    }

    /// 设置授权码
    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.request.code = Some(code.into());
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
        self
    }

    /// 设置权限范围
    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    /// 设置登录类型
    pub fn login_type(mut self, login_type: LoginType) -> Self {
        self.request.login_type = Some(login_type);
        self
    }

    /// 设置验证码
    pub fn captcha<S: Into<String>>(mut self, captcha: S) -> Self {
        self.request.captcha = Some(captcha.into());
        self
    }

    /// 设置客户端信息
    pub fn client_info(mut self, client_info: ClientInfo) -> Self {
        self.request.client_info = Some(client_info);
        self
    }

    /// 设置记住我
    pub fn remember_me(mut self, remember_me: bool) -> Self {
        self.request.remember_me = Some(remember_me);
        self
    }

    /// 构建登录请求
    pub fn build(self) -> LoginRequest {
        self.request
    }
}

impl Default for LoginRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户登录API
///
/// # auth_login_v1
///
/// 用户登录认证，支持多种登录方式。
///
/// ## 请求参数
///
/// - `grant_type`: 授权类型，默认为 `authorization_code`
/// - `client_id`: 客户端ID
/// - `client_secret`: 客户端密钥
/// - `code`: 授权码（用于授权码模式）
/// - `redirect_uri`: 重定向URI
/// - `refresh_token`: 刷新令牌（用于刷新令牌模式）
/// - `scope`: 权限范围
/// - `app_id`: 应用ID（用于应用密钥模式）
/// - `app_secret`: 应用密钥（用于应用密钥模式）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// // 授权码模式登录
/// let request = LoginRequestBuilder::new()
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .code("auth_code_here")
///     .redirect_uri("https://your-app.com/callback")
///     .build();
///
/// // 应用密钥模式登录
/// let request = LoginRequestBuilder::new()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .build();
/// ```
///
/// ## 返回值
///
/// 返回 `LoginResponse`，包含访问令牌、用户信息等。
pub async fn auth_login_v1(request: LoginRequest) -> SecurityResult<ServiceLoginResponse> {
    info!("处理用户登录请求: grant_type={:?}", request.grant_type);

    // 验证必要参数
    validate_login_request(&request)?;

    // 根据授权类型处理登录
    let response = match request.grant_type {
        GrantType::AuthorizationCode => handle_authorization_code_login(&request).await?,
        GrantType::RefreshToken => handle_refresh_token_login(&request).await?,
        GrantType::ClientCredentials => handle_client_credentials_login(&request).await?,
    };

    info!("用户登录成功: user_id={}", response.user_id);
    Ok(response)
}

/// 验证码登录
///
/// # auth_login_code_v1
///
/// 通过手机验证码进行登录认证。
///
/// ## 请求参数
///
/// - `phone`: 手机号码
/// - `code`: 验证码
/// - `country_code`: 国家代码，默认为 "+86"
/// - `login_type`: 登录类型
/// - `client_info`: 客户端信息
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let response = auth_login_code_v1(LoginCodeRequest {
///     phone: "+8613800138000".to_string(),
///     code: "123456".to_string(),
///     country_code: "+86".to_string(),
///     login_type: Some(LoginType::VerificationCode),
///     client_info: Some(ClientInfo {
///         client_type: ClientType::Mobile,
///         client_version: "1.0.0".to_string(),
///         os: "iOS 15.0".to_string(),
///         device_id: "device_123".to_string(),
///         user_agent: "MyApp/1.0.0".to_string(),
///     }),
/// }).await?;
/// ```
pub async fn auth_login_code_v1(request: LoginCodeRequest) -> SecurityResult<LoginCodeResponse> {
    info!("处理验证码登录请求: phone={}", request.phone);

    // 验证手机号码格式
    if !is_valid_phone_number(&request.phone, &request.country_code) {
        return Err(SecurityError::InvalidParameter {
            parameter: "phone".to_string(),
            reason: "手机号码格式无效".to_string(),
        });
    }

    // 验证验证码格式
    if request.code.len() != 6 || !request.code.chars().all(|c| c.is_ascii_digit()) {
        return Err(SecurityError::InvalidParameter {
            parameter: "code".to_string(),
            reason: "验证码格式无效".to_string(),
        });
    }

    // 模拟验证码验证逻辑
    if !validate_verification_code(&request.phone, &request.code).await? {
        return Err(SecurityError::AuthenticationError {
            message: "验证码错误或已过期".to_string(),
            error_code: Some("INVALID_VERIFICATION_CODE".to_string()),
        });
    }

    // 查找或创建用户
    let user_info = find_or_create_user_by_phone(&request.phone).await?;

    // 生成令牌
    let (access_token, refresh_token) = generate_token_pair(&user_info.user_id)?;

    let response = LoginCodeResponse {
        success: true,
        user_id: user_info.user_id.clone(),
        access_token,
        refresh_token,
        token_type: TokenType::AccessToken,
        expires_in: 7200,
        scopes: vec![
            "contact:user.base:readonly".to_string(),
            "message:message.send".to_string(),
        ],
        user_info,
        is_new_user: false,
    };

    info!("验证码登录成功: user_id={}", response.user_id);
    Ok(response)
}

/// SSO登录
///
/// # auth_login_sso_v1
///
/// 通过单点登录进行身份认证。
///
/// ## 请求参数
///
/// - `sso_token`: SSO令牌
/// - `provider`: SSO提供商
/// - `client_info`: 客户端信息
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let response = auth_login_sso_v1(SSOLoginRequest {
///     sso_token: "sso_token_123".to_string(),
///     provider: SSOProvider::Feishu,
///     client_info: Some(ClientInfo {
///         client_type: ClientType::Web,
///         client_version: "1.0.0".to_string(),
///         os: "Windows 10".to_string(),
///         device_id: "device_123".to_string(),
///         user_agent: "Mozilla/5.0...".to_string(),
///     }),
/// }).await?;
/// ```
pub async fn auth_login_sso_v1(request: SSOLoginRequest) -> SecurityResult<SSOLoginResponse> {
    info!("处理SSO登录请求: provider={:?}", request.provider);

    // 验证SSO令牌
    let sso_user_info = validate_sso_token(&request.sso_token, &request.provider).await?;

    // 查找或创建本地用户
    let user_info = find_or_create_user_by_sso(&sso_user_info).await?;

    // 生成令牌
    let (access_token, refresh_token) = generate_token_pair(&user_info.user_id)?;

    let response = SSOLoginResponse {
        success: true,
        user_id: user_info.user_id.clone(),
        access_token,
        refresh_token,
        token_type: TokenType::AccessToken,
        expires_in: 7200,
        scopes: vec![
            "contact:user.base:readonly".to_string(),
            "message:message.send".to_string(),
        ],
        user_info,
        sso_provider: request.provider,
    };

    info!(
        "SSO登录成功: user_id={}, provider={:?}",
        response.user_id, response.sso_provider
    );
    Ok(response)
}

// ============ 辅助函数 ============

/// 验证登录请求参数
fn validate_login_request(request: &LoginRequest) -> SecurityResult<()> {
    match request.grant_type {
        GrantType::AuthorizationCode => {
            if request.client_id.is_empty() || request.client_secret.is_empty() {
                return Err(SecurityError::InvalidParameter {
                    parameter: "client_credentials".to_string(),
                    reason: "客户端ID和密钥不能为空".to_string(),
                });
            }
            if request.code.is_none() {
                return Err(SecurityError::InvalidParameter {
                    parameter: "code".to_string(),
                    reason: "授权码不能为空".to_string(),
                });
            }
        }
        GrantType::RefreshToken => {
            if request.refresh_token.is_none() {
                return Err(SecurityError::InvalidParameter {
                    parameter: "refresh_token".to_string(),
                    reason: "刷新令牌不能为空".to_string(),
                });
            }
        }
        GrantType::ClientCredentials => {
            if request.app_id.is_none() || request.app_secret.is_none() {
                return Err(SecurityError::InvalidParameter {
                    parameter: "app_credentials".to_string(),
                    reason: "应用ID和密钥不能为空".to_string(),
                });
            }
        }
    }
    Ok(())
}

/// 处理授权码模式登录
async fn handle_authorization_code_login(
    request: &LoginRequest,
) -> SecurityResult<ServiceLoginResponse> {
    // 模拟授权码验证
    info!(
        "验证授权码: {}",
        request.code.as_ref().unwrap_or(&"None".to_string())
    );

    // 模拟用户信息
    let now = Utc::now();
    let user_id = format!("user_{}", uuid::Uuid::new_v4());

    let user_info = UserInfo {
        user_id: user_id.clone(),
        username: format!("user_{}", &request.client_id[..8]),
        display_name: format!("用户{}", &request.client_id[..8]),
        email: format!("user_{}@example.com", &request.client_id[..8]),
        phone: Some("+86 138 0000 0000".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        nickname: format!("用户{}", &request.client_id[..8]),
        avatar: "https://example.com/avatar.jpg".to_string(),
        tenant_key: format!("tenant_{}", &request.client_id[..8]),
        department_ids: vec![format!("dept_{}", &request.client_id[..8])],
        position: "工程师".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: now,
        user_type: UserType::User,
        status: UserStatus::Active,
        created_at: now - chrono::Duration::days(30),
        last_login_at: Some(now),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    let (access_token, refresh_token) = generate_token_pair(&user_id)?;

    Ok(ServiceLoginResponse {
        success: true,
        user_id: user_id.clone(),
        access_token,
        refresh_token,
        token_type: TokenType::AccessToken,
        expires_in: 7200,
        scopes: request
            .scope
            .as_ref()
            .map(|s| vec![s.clone()])
            .unwrap_or_default(),
        user_info,
    })
}

/// 处理刷新令牌模式登录
async fn handle_refresh_token_login(
    request: &LoginRequest,
) -> SecurityResult<ServiceLoginResponse> {
    info!(
        "刷新令牌: {}",
        request
            .refresh_token
            .as_ref()
            .unwrap_or(&"None".to_string())
    );

    // 模拟刷新令牌验证
    let refresh_token = request.refresh_token.as_ref().unwrap();
    if !refresh_token.starts_with("refresh_token_") {
        return Err(SecurityError::TokenError {
            message: "无效的刷新令牌格式".to_string(),
            token_type: Some(TokenType::RefreshToken),
            expires_at: None,
        });
    }

    // 模拟用户信息获取
    let user_id = "user_refresh_mock".to_string();
    let now = Utc::now();

    let user_info = UserInfo {
        user_id: user_id.clone(),
        username: "refresh_user".to_string(),
        display_name: "刷新用户".to_string(),
        email: "refresh@example.com".to_string(),
        phone: None,
        avatar_url: None,
        nickname: "刷新用户".to_string(),
        avatar: "".to_string(),
        tenant_key: "tenant_refresh".to_string(),
        department_ids: vec!["dept_refresh".to_string()],
        position: "用户".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: now,
        user_type: UserType::User,
        status: UserStatus::Active,
        created_at: now - chrono::Duration::days(90),
        last_login_at: Some(now),
        permissions: vec!["read".to_string()],
    };

    let (access_token, new_refresh_token) = generate_token_pair(&user_id)?;

    Ok(ServiceLoginResponse {
        success: true,
        user_id: user_id.clone(),
        access_token,
        refresh_token: new_refresh_token,
        token_type: TokenType::AccessToken,
        expires_in: 7200,
        scopes: request
            .scope
            .as_ref()
            .map(|s| vec![s.clone()])
            .unwrap_or_default(),
        user_info,
    })
}

/// 处理应用凭证模式登录
async fn handle_client_credentials_login(
    request: &LoginRequest,
) -> SecurityResult<ServiceLoginResponse> {
    info!(
        "应用凭证登录: app_id={}",
        request.app_id.as_ref().unwrap_or(&"None".to_string())
    );

    // 模拟应用凭证验证
    let app_id = request.app_id.as_ref().unwrap();
    if !app_id.starts_with("app_") {
        return Err(SecurityError::AuthenticationError {
            message: "无效的应用ID".to_string(),
            error_code: Some("INVALID_APP_ID".to_string()),
        });
    }

    // 模拟应用信息
    let user_id = format!("app_{}", app_id);
    let now = Utc::now();

    let user_info = UserInfo {
        user_id: user_id.clone(),
        username: app_id.clone(),
        display_name: format!("应用{}", app_id),
        email: format!("{}@feishu.cn", app_id),
        phone: None,
        avatar_url: Some("https://example.com/app-avatar.jpg".to_string()),
        nickname: format!("应用{}", app_id),
        avatar: "https://example.com/app-avatar.jpg".to_string(),
        tenant_key: format!("tenant_{}", app_id),
        department_ids: vec!["dept_app".to_string()],
        position: "应用服务".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: now,
        user_type: UserType::Application,
        status: UserStatus::Active,
        created_at: now - chrono::Duration::days(180),
        last_login_at: Some(now),
        permissions: vec!["app".to_string()],
    };

    let (access_token, refresh_token) = generate_token_pair(&user_id)?;

    Ok(ServiceLoginResponse {
        success: true,
        user_id: user_id.clone(),
        access_token,
        refresh_token,
        token_type: TokenType::AccessToken,
        expires_in: 7200,
        scopes: request
            .scope
            .as_ref()
            .map(|s| vec![s.clone()])
            .unwrap_or_default(),
        user_info,
    })
}

/// 验证手机号码格式
fn is_valid_phone_number(phone: &str, country_code: &str) -> bool {
    // 简单的手机号码验证逻辑
    if country_code == "+86" {
        // 中国手机号码验证
        phone.len() == 11 && phone.chars().all(|c| c.is_ascii_digit())
    } else {
        // 其他国家的手机号码验证（简单实现）
        phone.len() >= 10 && phone.chars().all(|c| c.is_ascii_digit() || c == '+')
    }
}

/// 验证验证码
async fn validate_verification_code(phone: &str, code: &str) -> SecurityResult<bool> {
    // 这里应该调用短信/邮件服务验证码验证接口
    // 模拟验证逻辑：123456 为有效验证码
    if code == "123456" {
        info!("验证码验证成功: phone={}", phone);
        Ok(true)
    } else {
        warn!("验证码验证失败: phone={}, code={}", phone, code);
        Ok(false)
    }
}

/// 根据手机号查找或创建用户
async fn find_or_create_user_by_phone(phone: &str) -> SecurityResult<UserInfo> {
    let now = Utc::now();

    // 模拟用户查找和创建逻辑
    Ok(UserInfo {
        user_id: format!("phone_user_{}", phone),
        username: phone.to_string(),
        display_name: format!("手机用户{}", &phone[7..11]),
        email: format!("{}@mobile.com", phone),
        phone: Some(phone.to_string()),
        avatar_url: Some("https://example.com/mobile-avatar.jpg".to_string()),
        nickname: format!("手机用户{}", &phone[7..11]),
        avatar: "https://example.com/mobile-avatar.jpg".to_string(),
        tenant_key: format!("tenant_phone_{}", &phone[7..11]),
        department_ids: vec!["dept_mobile".to_string()],
        position: "移动用户".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: now,
        user_type: UserType::User,
        status: UserStatus::Active,
        created_at: now,
        last_login_at: Some(now),
        permissions: vec!["read".to_string(), "write".to_string()],
    })
}

/// 验证SSO令牌
async fn validate_sso_token(
    sso_token: &str,
    provider: &SSOProvider,
) -> SecurityResult<SSOUserInfo> {
    // 这里应该调用对应SSO提供商的令牌验证接口
    info!(
        "验证SSO令牌: provider={:?}, token_prefix={}",
        provider,
        &sso_token[..std::cmp::min(10, sso_token.len())]
    );

    // 模拟SSO令牌验证
    if sso_token.starts_with("sso_token_") {
        Ok(SSOUserInfo {
            sso_user_id: format!("sso_user_{}", &sso_token[10..]),
            email: format!("sso_{}@example.com", provider.to_string().to_lowercase()),
            name: "SSO用户".to_string(),
            avatar_url: Some("https://example.com/sso-avatar.jpg".to_string()),
            provider: provider.clone(),
        })
    } else {
        Err(SecurityError::AuthenticationError {
            message: "无效的SSO令牌".to_string(),
            error_code: Some("INVALID_SSO_TOKEN".to_string()),
        })
    }
}

/// 根据SSO用户信息查找或创建本地用户
async fn find_or_create_user_by_sso(sso_user: &SSOUserInfo) -> SecurityResult<UserInfo> {
    let now = Utc::now();

    // 模拟SSO用户查找和本地用户创建逻辑
    Ok(UserInfo {
        user_id: format!("sso_user_{}", sso_user.sso_user_id),
        username: sso_user.email.clone(),
        display_name: sso_user.name.clone(),
        email: sso_user.email.clone(),
        phone: None,
        avatar_url: sso_user.avatar_url.clone(),
        nickname: sso_user.name.clone(),
        avatar: sso_user
            .avatar_url
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone(),
        tenant_key: format!("tenant_sso_{}", sso_user.sso_user_id),
        department_ids: vec!["dept_sso".to_string()],
        position: "SSO用户".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: now,
        user_type: UserType::User,
        status: UserStatus::Active,
        created_at: now,
        last_login_at: Some(now),
        permissions: vec!["read".to_string(), "write".to_string()],
    })
}

/// 生成令牌对
fn generate_token_pair(user_id: &str) -> SecurityResult<(String, String)> {
    let access_token = format!("access_token_{}_{}", user_id, uuid::Uuid::new_v4());
    let refresh_token = format!("refresh_token_{}_{}", user_id, uuid::Uuid::new_v4());

    Ok((access_token, refresh_token))
}

// ============ 请求和响应模型定义 ============

/// 授权类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GrantType {
    /// 授权码模式
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    /// 刷新令牌模式
    #[serde(rename = "refresh_token")]
    RefreshToken,
    /// 客户端凭证模式
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}

/// SSO提供商
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SSOProvider {
    /// 飞书SSO
    #[serde(rename = "feishu")]
    Feishu,
    /// 企业微信
    #[serde(rename = "wework")]
    Wework,
    /// 钉钉
    #[serde(rename = "dingtalk")]
    Dingtalk,
    /// 自定义SSO
    #[serde(rename = "custom")]
    Custom(String),
}

impl std::fmt::Display for SSOProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSOProvider::Feishu => write!(f, "feishu"),
            SSOProvider::Wework => write!(f, "wework"),
            SSOProvider::Dingtalk => write!(f, "dingtalk"),
            SSOProvider::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// 授权类型
    pub grant_type: GrantType,
    /// 客户端ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 授权码
    pub code: Option<String>,
    /// 重定向URI
    pub redirect_uri: Option<String>,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 权限范围
    pub scope: Option<String>,
    /// 应用ID（客户端凭证模式）
    pub app_id: Option<String>,
    /// 应用密钥（客户端凭证模式）
    pub app_secret: Option<String>,
    /// 登录类型
    pub login_type: Option<LoginType>,
    /// 验证码
    pub captcha: Option<String>,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
    /// 记住我
    pub remember_me: Option<bool>,
}

/// 验证码登录请求
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCodeRequest {
    /// 手机号码
    pub phone: String,
    /// 验证码
    pub code: String,
    /// 国家代码
    #[serde(default = "default_country_code")]
    pub country_code: String,
    /// 登录类型
    pub login_type: Option<LoginType>,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
}

/// 验证码登录响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCodeResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: TokenType,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 用户信息
    pub user_info: UserInfo,
    /// 是否为新用户
    pub is_new_user: bool,
}

/// SSO登录请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SSOLoginRequest {
    /// SSO令牌
    pub sso_token: String,
    /// SSO提供商
    pub provider: SSOProvider,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
}

/// SSO登录响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SSOLoginResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: TokenType,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 用户信息
    pub user_info: UserInfo,
    /// SSO提供商
    pub sso_provider: SSOProvider,
}

/// SSO用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SSOUserInfo {
    /// SSO用户ID
    pub sso_user_id: String,
    /// 邮箱
    pub email: String,
    /// 姓名
    pub name: String,
    /// 头像URL
    pub avatar_url: Option<String>,
    /// SSO提供商
    pub provider: SSOProvider,
}
