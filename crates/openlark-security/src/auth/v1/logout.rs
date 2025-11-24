//! 登出认证API v1
//!
//! 提供用户登出相关的API接口实现。

use crate::auth::service::{ActiveSession, ClientInfo, LoginStatus, TokenValidationResponse};
use crate::error::{SecurityError, SecurityResult};
use crate::models::*;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// 用户登出请求构建器
#[derive(Debug, Clone)]
pub struct LogoutRequestBuilder {
    request: LogoutRequest,
}

impl LogoutRequestBuilder {
    /// 创建新的登出请求构建器
    pub fn new() -> Self {
        Self {
            request: LogoutRequest {
                access_token: String::new(),
                device_id: None,
                logout_all: Some(false),
                client_info: None,
            },
        }
    }

    /// 设置访问令牌
    pub fn access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.request.access_token = access_token.into();
        self
    }

    /// 设置设备ID
    pub fn device_id<S: Into<String>>(mut self, device_id: S) -> Self {
        self.request.device_id = Some(device_id.into());
        self
    }

    /// 设置登出所有设备
    pub fn logout_all(mut self, logout_all: bool) -> Self {
        self.request.logout_all = Some(logout_all);
        self
    }

    /// 设置客户端信息
    pub fn client_info(mut self, client_info: ClientInfo) -> Self {
        self.request.client_info = Some(client_info);
        self
    }

    /// 构建登出请求
    pub fn build(self) -> LogoutRequest {
        self.request
    }
}

impl Default for LogoutRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户登出API
///
/// # auth_logout_v1
///
/// 用户登出认证，使访问令牌失效。
///
/// ## 请求参数
///
/// - `access_token`: 访问令牌
/// - `device_id`: 设备ID（可选）
/// - `logout_all`: 是否登出所有设备（可选，默认false）
/// - `client_info`: 客户端信息（可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// // 登出当前设备
/// let request = LogoutRequestBuilder::new()
///     .access_token("your_access_token")
///     .device_id("device_123")
///     .build();
///
/// let response = auth_logout_v1(request).await?;
///
/// // 登出所有设备
/// let request = LogoutRequestBuilder::new()
///     .access_token("your_access_token")
///     .logout_all(true)
///     .build();
///
/// let response = auth_logout_v1(request).await?;
/// ```
///
/// ## 返回值
///
/// 返回 `LogoutResponse`，包含登出结果信息。
pub async fn auth_logout_v1(request: LogoutRequest) -> SecurityResult<LogoutResponse> {
    info!(
        "处理用户登出请求: token_prefix={}",
        &request.access_token[..std::cmp::min(10, request.access_token.len())]
    );

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 处理登出逻辑
    let response = if request.logout_all.unwrap_or(false) {
        // 登出所有设备
        logout_all_devices(&request).await?
    } else {
        // 登出当前设备
        logout_current_device(&request).await?
    };

    info!("用户登出成功");
    Ok(response)
}

/// 全端登出API
///
/// # auth_logout_all_v1
///
/// 登出用户的所有活跃会话，使所有设备上的令牌失效。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID
/// - `access_token`: 访问令牌（用于验证）
/// - `reason`: 登出原因（可选）
/// - `force`: 是否强制登出（可选，默认false）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let response = auth_logout_all_v1(LogoutAllRequest {
///     user_id: "user_123".to_string(),
///     access_token: "your_access_token".to_string(),
///     reason: Some("安全原因".to_string()),
///     force: Some(true),
/// }).await?;
/// ```
pub async fn auth_logout_all_v1(request: LogoutAllRequest) -> SecurityResult<LogoutAllResponse> {
    info!("处理全端登出请求: user_id={}", request.user_id);

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 获取用户的所有活跃会话
    let active_sessions = get_user_active_sessions(&request.user_id).await?;
    let total_sessions = active_sessions.len();

    // 强制登出所有会话
    let logout_results = force_logout_all_sessions(&request.user_id, &request.reason).await?;

    let success_count = logout_results.iter().filter(|r| r.success).count();

    let response = LogoutAllResponse {
        success: success_count == total_sessions,
        user_id: request.user_id.clone(),
        total_sessions: total_sessions as u32,
        success_sessions: success_count as u32,
        failed_sessions: (total_sessions - success_count) as u32,
        logout_reason: request.reason.unwrap_or("用户主动登出".to_string()),
        force: request.force.unwrap_or(false),
        details: logout_results,
    };

    info!(
        "全端登出完成: user_id={}, 成功={}/{}",
        response.user_id, response.success_sessions, response.total_sessions
    );
    Ok(response)
}

/// 设备登出API
///
/// # auth_logout_device_v1
///
/// 登出指定设备的用户会话。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID
/// - `device_id`: 设备ID
/// - `access_token`: 访问令牌（用于验证）
/// - `reason`: 登出原因（可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let response = auth_logout_device_v1(LogoutDeviceRequest {
///     user_id: "user_123".to_string(),
///     device_id: "device_abc123".to_string(),
///     access_token: "your_access_token".to_string(),
///     reason: Some("设备丢失".to_string()),
/// }).await?;
/// ```
pub async fn auth_logout_device_v1(
    request: LogoutDeviceRequest,
) -> SecurityResult<LogoutDeviceResponse> {
    info!(
        "处理设备登出请求: user_id={}, device_id={}",
        request.user_id, request.device_id
    );

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 检查设备会话是否存在
    let session_exists = check_device_session_exists(&request.user_id, &request.device_id).await?;

    if !session_exists {
        return Ok(LogoutDeviceResponse {
            success: true,
            user_id: request.user_id.clone(),
            device_id: request.device_id.clone(),
            message: "设备会话不存在或已过期".to_string(),
            logout_time: chrono::Utc::now(),
        });
    }

    // 执行设备登出
    let logout_time = chrono::Utc::now();
    let logout_success = logout_device_session(&request.user_id, &request.device_id).await?;

    let response = LogoutDeviceResponse {
        success: logout_success,
        user_id: request.user_id.clone(),
        device_id: request.device_id.clone(),
        message: if logout_success {
            "设备登出成功".to_string()
        } else {
            "设备登出失败".to_string()
        },
        logout_time,
    };

    info!(
        "设备登出完成: user_id={}, device_id={}, success={}",
        response.user_id, response.device_id, response.success
    );
    Ok(response)
}

/// 检查登录状态API
///
/// # auth_check_login_status_v1
///
/// 检查用户的当前登录状态。
///
/// ## 请求参数
///
/// - `access_token`: 访问令牌
/// - `device_id`: 设备ID（可选）
/// - `include_sessions`: 是否包含会话详情（可选，默认false）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let response = auth_check_login_status_v1(LoginStatusRequest {
///     access_token: "your_access_token".to_string(),
///     device_id: Some("device_123".to_string()),
///     include_sessions: Some(true),
/// }).await?;
/// ```
pub async fn auth_check_login_status_v1(
    request: LoginStatusRequest,
) -> SecurityResult<LoginStatusResponse> {
    info!(
        "检查登录状态: token_prefix={}",
        &request.access_token[..std::cmp::min(10, request.access_token.len())]
    );

    // 验证访问令牌
    let token_validation = validate_access_token_detailed(&request.access_token).await?;

    if !token_validation.valid {
        return Ok(LoginStatusResponse {
            is_logged_in: false,
            user_id: None,
            device_id: request.device_id.clone(),
            last_login_time: None,
            session_count: 0,
            active_sessions: vec![],
            token_expires_at: token_validation.expires_at,
            is_device_trusted: false,
            security_warnings: vec![],
        });
    }

    // 获取用户登录状态
    let login_status = get_user_login_status(&token_validation.user_id.as_ref().unwrap()).await?;

    // 过滤设备会话（如果指定了设备ID）
    let active_sessions = if let Some(device_id) = &request.device_id {
        login_status
            .active_sessions
            .iter()
            .filter(|session| session.device_id == *device_id)
            .cloned()
            .collect()
    } else {
        login_status.active_sessions
    };

    // 检查设备信任状态
    let is_device_trusted = if let Some(device_id) = &request.device_id {
        check_device_trusted(&token_validation.user_id.as_ref().unwrap(), device_id).await?
    } else {
        true
    };

    // 获取安全警告
    let security_warnings =
        get_security_warnings(&token_validation.user_id.as_ref().unwrap()).await?;

    let response = LoginStatusResponse {
        is_logged_in: true,
        user_id: token_validation.user_id,
        device_id: request.device_id,
        last_login_time: login_status.last_login_time,
        session_count: active_sessions.len() as u32,
        active_sessions: if request.include_sessions.unwrap_or(false) {
            active_sessions
        } else {
            vec![]
        },
        token_expires_at: token_validation.expires_at,
        is_device_trusted,
        security_warnings,
    };

    info!(
        "登录状态检查完成: user_id={}, logged_in={}, sessions={}",
        response.user_id.as_ref().unwrap_or(&"unknown".to_string()),
        response.is_logged_in,
        response.session_count
    );
    Ok(response)
}

// ============ 辅助函数 ============

/// 验证访问令牌
async fn validate_access_token(access_token: &str) -> SecurityResult<()> {
    // 这里应该调用令牌验证服务
    // 模拟验证逻辑
    if access_token.is_empty() {
        return Err(SecurityError::AuthenticationError {
            message: "访问令牌不能为空".to_string(),
            error_code: Some("MISSING_ACCESS_TOKEN".to_string()),
        });
    }

    if !access_token.starts_with("access_token_") {
        return Err(SecurityError::AuthenticationError {
            message: "无效的访问令牌格式".to_string(),
            error_code: Some("INVALID_ACCESS_TOKEN".to_string()),
        });
    }

    // 检查令牌是否在黑名单中
    if is_token_blacklisted(access_token).await? {
        return Err(SecurityError::AuthenticationError {
            message: "访问令牌已被吊销".to_string(),
            error_code: Some("TOKEN_REVOKED".to_string()),
        });
    }

    Ok(())
}

/// 详细验证访问令牌
async fn validate_access_token_detailed(
    access_token: &str,
) -> SecurityResult<TokenValidationResponse> {
    validate_access_token(access_token).await?;

    // 模拟详细的令牌验证
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::hours(2);

    Ok(TokenValidationResponse {
        valid: true,
        user_id: Some(format!("user_{}", &access_token[13..21])),
        expires_at: Some(expires_at),
        scopes: vec![
            "contact:user.base:readonly".to_string(),
            "message:message.send".to_string(),
        ],
        error: None,
    })
}

/// 登出当前设备
async fn logout_current_device(request: &LogoutRequest) -> SecurityResult<LogoutResponse> {
    // 模拟设备登出逻辑
    info!("登出当前设备: device_id={:?}", request.device_id);

    // 在实际实现中，应该：
    // 1. 将令牌加入黑名单
    // 2. 清除设备相关的会话缓存
    // 3. 记录登出日志

    Ok(LogoutResponse {
        success: true,
        message: "当前设备登出成功".to_string(),
    })
}

/// 登出所有设备
async fn logout_all_devices(request: &LogoutRequest) -> SecurityResult<LogoutResponse> {
    // 模拟全设备登出逻辑
    info!("登出所有设备");

    // 在实际实现中，应该：
    // 1. 将用户所有令牌加入黑名单
    // 2. 清除用户所有会话缓存
    // 3. 记录全设备登出日志
    // 4. 通知其他在线设备

    Ok(LogoutResponse {
        success: true,
        message: "所有设备登出成功".to_string(),
    })
}

/// 获取用户活跃会话
async fn get_user_active_sessions(user_id: &str) -> SecurityResult<Vec<ActiveSession>> {
    // 模拟获取用户活跃会话
    let sessions = vec![
        ActiveSession {
            session_id: format!("session_{}", uuid::Uuid::new_v4()),
            device_id: "device_001".to_string(),
            device_type: DeviceType::Desktop,
            ip_address: "192.168.1.100".to_string(),
            location: "办公室".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::hours(2),
            last_activity: chrono::Utc::now() - chrono::Duration::minutes(15),
        },
        ActiveSession {
            session_id: format!("session_{}", uuid::Uuid::new_v4()),
            device_id: "device_002".to_string(),
            device_type: DeviceType::Mobile,
            ip_address: "192.168.1.101".to_string(),
            location: "家".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::hours(8),
            last_activity: chrono::Utc::now() - chrono::Duration::hours(1),
        },
    ];

    Ok(sessions)
}

/// 强制登出所有会话
async fn force_logout_all_sessions(
    user_id: &str,
    reason: &Option<String>,
) -> SecurityResult<Vec<SessionLogoutResult>> {
    let sessions = get_user_active_sessions(user_id).await?;
    let mut results = Vec::new();

    for session in sessions {
        // 模拟强制登出每个会话
        let logout_result = SessionLogoutResult {
            session_id: session.session_id.clone(),
            device_id: session.device_id.clone(),
            success: true,
            error: None,
            logout_time: chrono::Utc::now(),
        };
        results.push(logout_result);
    }

    Ok(results)
}

/// 检查设备会话是否存在
async fn check_device_session_exists(user_id: &str, device_id: &str) -> SecurityResult<bool> {
    let sessions = get_user_active_sessions(user_id).await?;
    Ok(sessions
        .iter()
        .any(|session| session.device_id == device_id))
}

/// 登出设备会话
async fn logout_device_session(user_id: &str, device_id: &str) -> SecurityResult<bool> {
    // 模拟登出设备会话
    info!("登出设备会话: user_id={}, device_id={}", user_id, device_id);

    // 在实际实现中，应该：
    // 1. 移除设备会话
    // 2. 将相关令牌加入黑名单
    // 3. 记录设备登出日志

    Ok(true)
}

/// 获取用户登录状态
async fn get_user_login_status(user_id: &str) -> SecurityResult<LoginStatus> {
    // 模拟获取用户登录状态
    Ok(LoginStatus {
        user_id: user_id.to_string(),
        is_logged_in: true,
        last_login_time: Some(chrono::Utc::now() - chrono::Duration::minutes(30)),
        session_count: 2,
        active_sessions: get_user_active_sessions(user_id).await?,
        is_locked: false,
        locked_reason: None,
        locked_until: None,
    })
}

/// 检查设备信任状态
async fn check_device_trusted(user_id: &str, device_id: &str) -> SecurityResult<bool> {
    // 模拟设备信任检查
    // 可以基于设备ID、用户ID、IP地址等判断
    let trusted_devices = vec!["device_001".to_string(), "device_002".to_string()];
    Ok(trusted_devices.contains(&device_id.to_string()))
}

/// 获取安全警告
async fn get_security_warnings(user_id: &str) -> SecurityResult<Vec<SecurityWarning>> {
    // 模拟安全警告检查
    let mut warnings = Vec::new();

    // 检查是否有异常登录
    if has_anomalous_login(user_id).await? {
        warnings.push(SecurityWarning {
            warning_type: "ANOMALOUS_LOGIN".to_string(),
            message: "检测到异常登录行为".to_string(),
            severity: SecurityWarningSeverity::Medium,
            created_at: chrono::Utc::now(),
        });
    }

    // 检查是否有多设备登录
    let active_sessions = get_user_active_sessions(user_id).await?;
    if active_sessions.len() > 3 {
        warnings.push(SecurityWarning {
            warning_type: "MULTI_DEVICE_LOGIN".to_string(),
            message: "检测到多个设备同时登录".to_string(),
            severity: SecurityWarningSeverity::Low,
            created_at: chrono::Utc::now(),
        });
    }

    Ok(warnings)
}

/// 检查令牌是否在黑名单中
async fn is_token_blacklisted(access_token: &str) -> SecurityResult<bool> {
    // 模拟黑名单检查
    // 在实际实现中，应该检查Redis或数据库中的黑名单
    let blacklisted_tokens = vec!["blacklisted_token_1".to_string()];
    Ok(blacklisted_tokens.contains(&access_token.to_string()))
}

/// 检测异常登录行为
async fn has_anomalous_login(user_id: &str) -> SecurityResult<bool> {
    // 模拟异常登录检测
    // 可以基于登录频率、地理位置变化、设备变化等检测
    Ok(false) // 简单实现，实际中应该有更复杂的逻辑
}

// ============ 请求和响应模型定义 ============

/// 登出请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// 访问令牌
    pub access_token: String,
    /// 设备ID
    pub device_id: Option<String>,
    /// 登出所有设备
    pub logout_all: Option<bool>,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
}

/// 全端登出请求
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutAllRequest {
    /// 用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 登出原因
    pub reason: Option<String>,
    /// 是否强制登出
    pub force: Option<bool>,
}

/// 设备登出请求
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutDeviceRequest {
    /// 用户ID
    pub user_id: String,
    /// 设备ID
    pub device_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 登出原因
    pub reason: Option<String>,
}

/// 登出响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: String,
}

/// 全端登出响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutAllResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 总会话数
    pub total_sessions: u32,
    /// 成功登出会话数
    pub success_sessions: u32,
    /// 失败登出会话数
    pub failed_sessions: u32,
    /// 登出原因
    pub logout_reason: String,
    /// 是否强制登出
    pub force: bool,
    /// 详细结果
    pub details: Vec<SessionLogoutResult>,
}

/// 设备登出响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutDeviceResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 设备ID
    pub device_id: String,
    /// 消息
    pub message: String,
    /// 登出时间
    pub logout_time: chrono::DateTime<chrono::Utc>,
}

/// 会话登出结果
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionLogoutResult {
    /// 会话ID
    pub session_id: String,
    /// 设备ID
    pub device_id: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
    /// 登出时间
    pub logout_time: chrono::DateTime<chrono::Utc>,
}

/// 登录状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginStatusRequest {
    /// 访问令牌
    pub access_token: String,
    /// 设备ID
    pub device_id: Option<String>,
    /// 是否包含会话详情
    pub include_sessions: Option<bool>,
}

/// 登录状态响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginStatusResponse {
    /// 是否已登录
    pub is_logged_in: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 设备ID
    pub device_id: Option<String>,
    /// 最后登录时间
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 活跃会话数量
    pub session_count: u32,
    /// 活跃会话列表
    pub active_sessions: Vec<ActiveSession>,
    /// 令牌过期时间
    pub token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 设备是否受信任
    pub is_device_trusted: bool,
    /// 安全警告
    pub security_warnings: Vec<SecurityWarning>,
}

/// 安全警告
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityWarning {
    /// 警告类型
    pub warning_type: String,
    /// 警告消息
    pub message: String,
    /// 警告级别
    pub severity: SecurityWarningSeverity,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 安全警告级别
#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityWarningSeverity {
    /// 低级
    #[serde(rename = "low")]
    Low,
    /// 中级
    #[serde(rename = "medium")]
    Medium,
    /// 高级
    #[serde(rename = "high")]
    High,
    /// 严重
    #[serde(rename = "critical")]
    Critical,
}
