//! 用户信息管理API v1
//!
//! 提供用户身份认证和用户信息获取功能。

use crate::auth::service::TokenValidationResponse;
use crate::error::{SecurityError, SecurityResult};
use crate::models::*;
use crate::models::{TokenType, UserType};
use crate::service::{UserInfo, UserStatus};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 用户身份验证请求构建器
#[derive(Debug, Clone)]
pub struct UserAuthRequestBuilder {
    request: UserAuthRequest,
}

impl UserAuthRequestBuilder {
    /// 创建新的用户身份验证请求构建器
    pub fn new() -> Self {
        Self {
            request: UserAuthRequest {
                user_id: None,
                email: None,
                phone: None,
                username: None,
                access_token: String::new(),
                verify_method: "access_token".to_string(),
                include_profile: Some(false),
                include_permissions: Some(false),
                device_id: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id<S: Into<String>>(mut self, user_id: S) -> Self {
        self.request.user_id = Some(user_id.into());
        self
    }

    /// 设置邮箱
    pub fn email<S: Into<String>>(mut self, email: S) -> Self {
        self.request.email = Some(email.into());
        self
    }

    /// 设置手机号
    pub fn phone<S: Into<String>>(mut self, phone: S) -> Self {
        self.request.phone = Some(phone.into());
        self
    }

    /// 设置用户名
    pub fn username<S: Into<String>>(mut self, username: S) -> Self {
        self.request.username = Some(username.into());
        self
    }

    /// 设置访问令牌
    pub fn access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.request.access_token = access_token.into();
        self
    }

    /// 设置验证方法
    pub fn verify_method<S: Into<String>>(mut self, verify_method: S) -> Self {
        self.request.verify_method = verify_method.into();
        self
    }

    /// 设置是否包含个人资料
    pub fn include_profile(mut self, include_profile: bool) -> Self {
        self.request.include_profile = Some(include_profile);
        self
    }

    /// 设置是否包含权限信息
    pub fn include_permissions(mut self, include_permissions: bool) -> Self {
        self.request.include_permissions = Some(include_permissions);
        self
    }

    /// 设置设备ID
    pub fn device_id<S: Into<String>>(mut self, device_id: S) -> Self {
        self.request.device_id = Some(device_id.into());
        self
    }

    /// 构建用户身份验证请求
    pub fn build(self) -> UserAuthRequest {
        self.request
    }
}

impl Default for UserAuthRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户信息更新请求构建器
#[derive(Debug, Clone)]
pub struct UserUpdateRequestBuilder {
    request: UserUpdateRequest,
}

impl UserUpdateRequestBuilder {
    /// 创建新的用户信息更新请求构建器
    pub fn new() -> Self {
        Self {
            request: UserUpdateRequest {
                user_id: String::new(),
                access_token: String::new(),
                nickname: None,
                avatar: None,
                phone: None,
                email: None,
                department_ids: None,
                position: None,
                employee_number: None,
                work_station: None,
                mobile_visible: None,
                email_visible: None,
                gender: None,
                is_avatar_in_gravatar: None,
                is_senior: None,
                roles: None,
                custom_attrs: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id<S: Into<String>>(mut self, user_id: S) -> Self {
        self.request.user_id = user_id.into();
        self
    }

    /// 设置访问令牌
    pub fn access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.request.access_token = access_token.into();
        self
    }

    /// 设置昵称
    pub fn nickname<S: Into<String>>(mut self, nickname: S) -> Self {
        self.request.nickname = Some(nickname.into());
        self
    }

    /// 设置头像
    pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
        self.request.avatar = Some(avatar.into());
        self
    }

    /// 设置手机号
    pub fn phone<S: Into<String>>(mut self, phone: S) -> Self {
        self.request.phone = Some(phone.into());
        self
    }

    /// 设置邮箱
    pub fn email<S: Into<String>>(mut self, email: S) -> Self {
        self.request.email = Some(email.into());
        self
    }

    /// 设置部门ID列表
    pub fn department_ids<I, S>(mut self, department_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.request.department_ids =
            Some(department_ids.into_iter().map(|id| id.into()).collect());
        self
    }

    /// 设置职位
    pub fn position<S: Into<String>>(mut self, position: S) -> Self {
        self.request.position = Some(position.into());
        self
    }

    /// 设置工号
    pub fn employee_number<S: Into<String>>(mut self, employee_number: S) -> Self {
        self.request.employee_number = Some(employee_number.into());
        self
    }

    /// 设置工位
    pub fn work_station<S: Into<String>>(mut self, work_station: S) -> Self {
        self.request.work_station = Some(work_station.into());
        self
    }

    /// 设置手机号可见性
    pub fn mobile_visible(mut self, mobile_visible: bool) -> Self {
        self.request.mobile_visible = Some(mobile_visible);
        self
    }

    /// 设置邮箱可见性
    pub fn email_visible(mut self, email_visible: bool) -> Self {
        self.request.email_visible = Some(email_visible);
        self
    }

    /// 设置性别
    pub fn gender<S: Into<String>>(mut self, gender: S) -> Self {
        self.request.gender = Some(gender.into());
        self
    }

    /// 构建用户信息更新请求
    pub fn build(self) -> UserUpdateRequest {
        self.request
    }
}

impl Default for UserUpdateRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户身份验证API
///
/// # auth_user_info_v1
///
/// 验证用户身份并获取用户基本信息。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID（可选，与email、phone、username至少提供一个）
/// - `email`: 邮箱（可选）
/// - `phone`: 手机号（可选）
/// - `username`: 用户名（可选）
/// - `access_token`: 访问令牌
/// - `verify_method`: 验证方法（access_token、session_id等）
/// - `include_profile`: 是否包含个人资料（可选，默认false）
/// - `include_permissions`: 是否包含权限信息（可选，默认false）
/// - `device_id`: 设备ID（可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// // 通过访问令牌验证
/// let request = UserAuthRequestBuilder::new()
///     .access_token("your_access_token")
///     .include_profile(true)
///     .include_permissions(true)
///     .build();
///
/// let response = auth_user_info_v1(request).await?;
///
/// // 通过用户ID和访问令牌验证
/// let request = UserAuthRequestBuilder::new()
///     .user_id("user_123")
///     .access_token("your_access_token")
///     .device_id("device_001")
///     .build();
///
/// let response = auth_user_info_v1(request).await?;
/// ```
pub async fn auth_user_info_v1(request: UserAuthRequest) -> SecurityResult<UserAuthResponse> {
    info!(
        "用户身份验证: verify_method={}, user_id={:?}, email={:?}",
        request.verify_method, request.user_id, request.email
    );

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 确定用户标识符
    let user_identifier = determine_user_identifier(&request)?;

    // 验证用户身份
    let user_info = verify_user_identity(&user_identifier, &request).await?;

    // 获取用户详细信息
    let user_profile = if request.include_profile.unwrap_or(false) {
        Some(get_user_profile(&user_info.user_id).await?)
    } else {
        None
    };

    // 获取用户权限
    let user_permissions = if request.include_permissions.unwrap_or(false) {
        Some(get_user_permissions(&user_info.user_id).await?)
    } else {
        None
    };

    // 检查设备信任状态
    let device_trusted = if let Some(device_id) = &request.device_id {
        check_device_trusted(&user_info.user_id, device_id).await?
    } else {
        true
    };

    let response = UserAuthResponse {
        success: true,
        user_id: user_info.user_id,
        username: Some(user_info.username),
        email: Some(user_info.email),
        phone: user_info.phone,
        nickname: Some(user_info.nickname),
        avatar: Some(user_info.avatar),
        status: format!("{:?}", user_info.status),
        tenant_key: Some(user_info.tenant_key),
        department_ids: Some(user_info.department_ids.clone()),
        position: Some(user_info.position),
        is_active: user_info.is_active,
        is_admin: user_info.is_admin,
        created_at: user_info.created_at,
        updated_at: user_info.updated_at,
        last_login_at: user_info.last_login_at,
        profile: user_profile,
        permissions: user_permissions,
        device_trusted,
        verified_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + Duration::hours(24),
    };

    info!(
        "用户身份验证成功: user_id={}, username={}",
        response.user_id,
        response.username.as_ref().unwrap_or(&"unknown".to_string())
    );
    Ok(response)
}

/// 获取用户详细信息API
///
/// # auth_get_user_detail_v1
///
/// 获取用户的详细信息，包括个人资料、权限、角色等。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID
/// - `access_token`: 访问令牌
/// - `include_sensitive`: 是否包含敏感信息（可选，默认false）
/// - `include_permissions`: 是否包含权限信息（可选，默认true）
/// - `include_roles`: 是否包含角色信息（可选，默认true）
/// - `include_departments`: 是否包含部门信息（可选，默认true）
/// - `include_login_history`: 是否包含登录历史（可选，默认false）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = UserDetailRequest {
///     user_id: "user_123".to_string(),
///     access_token: "your_access_token".to_string(),
///     include_sensitive: Some(false),
///     include_permissions: Some(true),
///     include_roles: Some(true),
///     include_departments: Some(true),
///     include_login_history: Some(false),
/// };
///
/// let response = auth_get_user_detail_v1(request).await?;
/// ```
pub async fn auth_get_user_detail_v1(
    request: UserDetailRequest,
) -> SecurityResult<UserDetailResponse> {
    info!("获取用户详细信息: user_id={}", request.user_id);

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 验证权限（用户只能查看自己的信息或有管理员权限）
    let token_validation = validate_access_token_detailed(&request.access_token).await?;
    let current_user_id = token_validation.user_id.as_ref().unwrap();

    if current_user_id != &request.user_id && !has_admin_permission(current_user_id).await? {
        return Err(SecurityError::PermissionDenied {
            resource: "user_detail".to_string(),
            action: "read".to_string(),
            message: "没有权限查看其他用户的详细信息".to_string(),
        });
    }

    // 获取用户基本信息
    let user_info = get_user_info(&request.user_id).await?;

    // 获取用户详细资料
    let profile = get_user_profile(&request.user_id).await?;

    // 获取用户权限
    let permissions = if request.include_permissions.unwrap_or(true) {
        Some(get_user_permissions(&request.user_id).await?)
    } else {
        None
    };

    // 获取用户角色
    let roles = if request.include_roles.unwrap_or(true) {
        Some(get_user_roles(&request.user_id).await?)
    } else {
        None
    };

    // 获取部门信息
    let departments = if request.include_departments.unwrap_or(true) {
        Some(get_user_departments(&Some(user_info.department_ids.clone())).await?)
    } else {
        None
    };

    // 获取登录历史
    let login_history = if request.include_login_history.unwrap_or(false) {
        Some(get_user_login_history(&request.user_id, 10).await?)
    } else {
        None
    };

    // 获取敏感信息
    let sensitive_info = if request.include_sensitive.unwrap_or(false) {
        Some(get_user_sensitive_info(&request.user_id).await?)
    } else {
        None
    };

    let response = UserDetailResponse {
        user_id: user_info.user_id,
        username: Some(user_info.username),
        email: Some(user_info.email),
        phone: user_info.phone,
        nickname: Some(user_info.nickname),
        avatar: Some(user_info.avatar),
        status: format!("{:?}", user_info.status),
        tenant_key: Some(user_info.tenant_key),
        employee_number: profile.employee_number,
        employee_type: profile.employee_type,
        join_time: profile.join_time,
        work_station: profile.work_station,
        position: profile.position,
        direct_manager: profile.direct_manager,
        department_ids: Some(user_info.department_ids.clone()),
        departments,
        roles,
        permissions,
        is_active: user_info.is_active,
        is_admin: user_info.is_admin,
        is_senior: profile.is_senior,
        created_at: user_info.created_at,
        updated_at: user_info.updated_at,
        last_login_at: user_info.last_login_at,
        login_history,
        sensitive_info,
        retrieved_at: chrono::Utc::now(),
    };

    info!(
        "用户详细信息获取成功: user_id={}, username={}",
        response.user_id,
        response.username.as_ref().unwrap_or(&"unknown".to_string())
    );
    Ok(response)
}

/// 更新用户信息API
///
/// # auth_update_user_info_v1
///
/// 更新用户的基本信息。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID
/// - `access_token`: 访问令牌
/// - `nickname`: 昵称（可选）
/// - `avatar`: 头像（可选）
/// - `phone`: 手机号（可选）
/// - `email`: 邮箱（可选）
/// - `department_ids`: 部门ID列表（可选）
/// - `position`: 职位（可选）
/// - `employee_number`: 工号（可选）
/// - `work_station`: 工位（可选）
/// - `mobile_visible`: 手机号可见性（可选）
/// - `email_visible`: 邮箱可见性（可选）
/// - `gender`: 性别（可选）
/// - `is_avatar_in_gravatar`: 是否使用gravatar头像（可选）
/// - `is_senior`: 是否是高级用户（可选）
/// - `roles`: 角色列表（可选）
/// - `custom_attrs`: 自定义属性（可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = UserUpdateRequestBuilder::new()
///     .user_id("user_123")
///     .access_token("your_access_token")
///     .nickname("新昵称")
///     .position("高级工程师")
///     .email_visible(true)
///     .build();
///
/// let response = auth_update_user_info_v1(request).await?;
/// ```
pub async fn auth_update_user_info_v1(
    request: UserUpdateRequest,
) -> SecurityResult<UpdateUserResponse> {
    info!("更新用户信息: user_id={}", request.user_id);

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 验证权限（用户只能更新自己的信息或有管理员权限）
    let token_validation = validate_access_token_detailed(&request.access_token).await?;
    let current_user_id = token_validation.user_id.as_ref().unwrap();

    if current_user_id != &request.user_id && !has_admin_permission(current_user_id).await? {
        return Err(SecurityError::PermissionDenied {
            resource: "user_info".to_string(),
            action: "update".to_string(),
            message: "没有权限更新其他用户的信息".to_string(),
        });
    }

    // 验证更新数据
    validate_update_data(&request).await?;

    // 执行更新操作
    let updated_fields = update_user_info_internal(&request).await?;

    // 获取更新后的用户信息
    let updated_user_info = get_user_info(&request.user_id).await?;

    let response = UpdateUserResponse {
        success: true,
        user_id: request.user_id,
        update_time: chrono::Utc::now(),
        updated_fields,
        message: "用户信息更新成功".to_string(),
    };

    info!(
        "用户信息更新成功: user_id={}, updated_fields={}, update_time={}",
        response.user_id,
        response.updated_fields.len(),
        response.update_time
    );
    Ok(response)
}

/// 检查用户权限API
///
/// # auth_check_user_permission_v1
///
/// 检查用户是否具有指定权限。
///
/// ## 请求参数
///
/// - `user_id`: 用户ID（可选，默认为当前用户）
/// - `access_token`: 访问令牌
/// - `permission`: 权限标识符
/// - `resource_type`: 资源类型（可选）
/// - `resource_id`: 资源ID（可选）
/// - `context`: 权限检查上下文（可选）
///
/// ## 使用方法
///
/// ```rust
/// use openlark_security::auth::v1::*;
///
/// let request = PermissionCheckRequest {
///     user_id: None, // 检查当前用户
///     access_token: "your_access_token".to_string(),
///     permission: "contact:user.base:readonly".to_string(),
///     resource_type: Some("contact".to_string()),
///     resource_id: Some("user_123".to_string()),
///     context: Some("read_user_profile".to_string()),
/// };
///
/// let response = auth_check_user_permission_v1(request).await?;
/// ```
pub async fn auth_check_user_permission_v1(
    request: PermissionCheckRequest,
) -> SecurityResult<PermissionCheckResponse> {
    info!(
        "检查用户权限: permission={}, resource_type={:?}",
        request.permission, request.resource_type
    );

    // 验证访问令牌
    validate_access_token(&request.access_token).await?;

    // 获取用户ID
    let target_user_id = if let Some(user_id) = request.user_id {
        // 检查指定用户的权限需要管理员权限
        let token_validation = validate_access_token_detailed(&request.access_token).await?;
        let current_user_id = token_validation.user_id.as_ref().unwrap();

        if current_user_id != &user_id && !has_admin_permission(current_user_id).await? {
            return Err(SecurityError::PermissionDenied {
                resource: "user_permission".to_string(),
                action: "check".to_string(),
                message: "没有权限检查其他用户的权限".to_string(),
            });
        }

        user_id
    } else {
        // 检查当前用户的权限
        let token_validation = validate_access_token_detailed(&request.access_token).await?;
        token_validation.user_id.unwrap()
    };

    // 获取用户所有权限
    let user_permissions = get_user_permissions(&target_user_id).await?;

    // 检查指定权限
    let permission_check = check_specific_permission(
        &user_permissions,
        &request.permission,
        &request.resource_type,
        &request.resource_id,
        &request.context,
    )
    .await?;

    let response = PermissionCheckResponse {
        user_id: target_user_id,
        permission: request.permission.clone(),
        resource_type: request.resource_type,
        resource_id: request.resource_id,
        context: request.context,
        has_permission: permission_check.has_permission,
        granted_at: permission_check.granted_at,
        expires_at: permission_check.expires_at,
        granted_by: permission_check.granted_by,
        restrictions: permission_check.restrictions,
        checked_at: chrono::Utc::now(),
        cached: permission_check.cached,
    };

    info!(
        "权限检查完成: user_id={}, permission={}, has_permission={}",
        response.user_id, response.permission, response.has_permission
    );
    Ok(response)
}

// ============ 辅助函数 ============

/// 验证访问令牌
async fn validate_access_token(access_token: &str) -> SecurityResult<()> {
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

    // 在实际实现中，应该验证令牌的有效性和过期时间
    Ok(())
}

/// 详细验证访问令牌
async fn validate_access_token_detailed(
    access_token: &str,
) -> SecurityResult<TokenValidationResponse> {
    validate_access_token(access_token).await?;

    // 模拟详细的令牌验证
    Ok(TokenValidationResponse {
        valid: true,
        user_id: Some("user_12345".to_string()),
        expires_at: Some(chrono::Utc::now() + Duration::hours(2)),
        scopes: vec![
            "contact:user.base:readonly".to_string(),
            "message:message.send".to_string(),
        ],
        error: None,
    })
}

/// 确定用户标识符
fn determine_user_identifier(request: &UserAuthRequest) -> SecurityResult<String> {
    if let Some(user_id) = &request.user_id {
        return Ok(user_id.clone());
    }

    if let Some(email) = &request.email {
        return Ok(format!("email:{}", email));
    }

    if let Some(phone) = &request.phone {
        return Ok(format!("phone:{}", phone));
    }

    if let Some(username) = &request.username {
        return Ok(format!("username:{}", username));
    }

    Err(SecurityError::MissingParameter {
        parameter: "user_identifier".to_string(),
        message: "必须提供用户ID、邮箱、手机号或用户名中的一个".to_string(),
    })
}

/// 验证用户身份
async fn verify_user_identity(
    user_identifier: &str,
    request: &UserAuthRequest,
) -> SecurityResult<UserInfo> {
    // 模拟用户身份验证
    let user_id = if user_identifier.starts_with("email:") {
        "user_12345".to_string()
    } else if user_identifier.starts_with("phone:") {
        "user_12345".to_string()
    } else if user_identifier.starts_with("username:") {
        "user_12345".to_string()
    } else {
        user_identifier.to_string()
    };

    // 获取用户信息
    get_user_info(&user_id).await
}

/// 获取用户信息
async fn get_user_info(user_id: &str) -> SecurityResult<UserInfo> {
    // 模拟获取用户信息
    Ok(UserInfo {
        user_id: user_id.to_string(),
        username: "john_doe".to_string(),
        display_name: "John Doe".to_string(),
        email: "john.doe@company.com".to_string(),
        phone: Some("13800138000".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        nickname: "John".to_string(),
        avatar: "https://example.com/avatar.jpg".to_string(),
        tenant_key: "tenant_abc".to_string(),
        department_ids: vec!["dept_001".to_string(), "dept_002".to_string()],
        position: "软件工程师".to_string(),
        is_active: true,
        is_admin: false,
        updated_at: chrono::Utc::now() - Duration::hours(1),
        user_type: UserType::User,
        status: UserStatus::Active,
        created_at: chrono::Utc::now() - Duration::days(365),
        last_login_at: Some(chrono::Utc::now() - Duration::minutes(30)),
        permissions: vec!["read".to_string()],
    })
}

/// 获取用户详细资料
async fn get_user_profile(user_id: &str) -> SecurityResult<UserProfile> {
    // 模拟获取用户详细资料
    Ok(UserProfile {
        employee_number: Some("EMP001".to_string()),
        employee_type: Some("正式员工".to_string()),
        join_time: Some(chrono::Utc::now() - Duration::days(365)),
        work_station: Some("工位A101".to_string()),
        position: Some("软件工程师".to_string()),
        direct_manager: Some("manager_001".to_string()),
        is_senior: Some(false),
        custom_attributes: std::collections::HashMap::new(),
    })
}

/// 获取用户权限
async fn get_user_permissions(user_id: &str) -> SecurityResult<Vec<UserPermission>> {
    // 模拟获取用户权限
    Ok(vec![
        UserPermission {
            permission_id: "perm_001".to_string(),
            permission_code: "contact:user.base:readonly".to_string(),
            permission_name: "联系人基础信息只读".to_string(),
            resource_type: "contact".to_string(),
            granted_at: chrono::Utc::now() - Duration::days(30),
            expires_at: Some(chrono::Utc::now() + Duration::days(365)),
            granted_by: Some("admin".to_string()),
            is_active: true,
        },
        UserPermission {
            permission_id: "perm_002".to_string(),
            permission_code: "message:message.send".to_string(),
            permission_name: "发送消息".to_string(),
            resource_type: "message".to_string(),
            granted_at: chrono::Utc::now() - Duration::days(30),
            expires_at: Some(chrono::Utc::now() + Duration::days(365)),
            granted_by: Some("admin".to_string()),
            is_active: true,
        },
    ])
}

/// 检查设备信任状态
async fn check_device_trusted(user_id: &str, device_id: &str) -> SecurityResult<bool> {
    // 模拟设备信任检查
    let trusted_devices = vec!["device_001".to_string(), "device_002".to_string()];
    Ok(trusted_devices.contains(&device_id.to_string()))
}

/// 检查管理员权限
async fn has_admin_permission(user_id: &str) -> SecurityResult<bool> {
    // 模拟管理员权限检查
    Ok(user_id == "admin_001" || user_id == "user_12345")
}

/// 验证更新数据
async fn validate_update_data(request: &UserUpdateRequest) -> SecurityResult<()> {
    // 验证邮箱格式
    if let Some(email) = &request.email {
        if !email.contains('@') || !email.contains('.') {
            return Err(SecurityError::InvalidParameter {
                parameter: "email".to_string(),
                reason: "邮箱格式无效".to_string(),
            });
        }
    }

    // 验证手机号格式
    if let Some(phone) = &request.phone {
        if phone.len() != 11 || !phone.chars().all(|c| c.is_ascii_digit()) {
            return Err(SecurityError::InvalidParameter {
                parameter: "phone".to_string(),
                reason: "手机号格式无效".to_string(),
            });
        }
    }

    Ok(())
}

/// 更新用户信息（内部）
async fn update_user_info_internal(request: &UserUpdateRequest) -> SecurityResult<Vec<String>> {
    let mut updated_fields = Vec::new();

    if request.nickname.is_some() {
        updated_fields.push("nickname".to_string());
    }
    if request.avatar.is_some() {
        updated_fields.push("avatar".to_string());
    }
    if request.phone.is_some() {
        updated_fields.push("phone".to_string());
    }
    if request.email.is_some() {
        updated_fields.push("email".to_string());
    }
    if request.department_ids.is_some() {
        updated_fields.push("department_ids".to_string());
    }
    if request.position.is_some() {
        updated_fields.push("position".to_string());
    }
    if request.employee_number.is_some() {
        updated_fields.push("employee_number".to_string());
    }
    if request.work_station.is_some() {
        updated_fields.push("work_station".to_string());
    }
    if request.mobile_visible.is_some() {
        updated_fields.push("mobile_visible".to_string());
    }
    if request.email_visible.is_some() {
        updated_fields.push("email_visible".to_string());
    }
    if request.gender.is_some() {
        updated_fields.push("gender".to_string());
    }
    if request.is_avatar_in_gravatar.is_some() {
        updated_fields.push("is_avatar_in_gravatar".to_string());
    }
    if request.is_senior.is_some() {
        updated_fields.push("is_senior".to_string());
    }

    info!(
        "更新用户信息: user_id={}, fields={:?}",
        request.user_id, updated_fields
    );

    // 在实际实现中，应该更新数据库中的用户信息
    Ok(updated_fields)
}

/// 获取用户角色
async fn get_user_roles(user_id: &str) -> SecurityResult<Vec<UserRole>> {
    // 模拟获取用户角色
    Ok(vec![UserRole {
        role_id: "role_001".to_string(),
        role_code: "developer".to_string(),
        role_name: "开发人员".to_string(),
        assigned_at: chrono::Utc::now() - Duration::days(60),
        assigned_by: Some("admin".to_string()),
        is_active: true,
    }])
}

/// 获取用户部门信息
async fn get_user_departments(
    department_ids: &Option<Vec<String>>,
) -> SecurityResult<Vec<DepartmentInfo>> {
    let mut departments = Vec::new();

    if let Some(dept_ids) = department_ids {
        for dept_id in dept_ids {
            departments.push(DepartmentInfo {
                department_id: dept_id.clone(),
                department_name: format!("部门_{}", &dept_id[5..]),
                parent_id: None,
                level: 1,
                member_count: 10,
                created_at: chrono::Utc::now() - Duration::days(730),
            });
        }
    }

    Ok(departments)
}

/// 获取用户登录历史
async fn get_user_login_history(user_id: &str, limit: i32) -> SecurityResult<Vec<LoginRecord>> {
    // 模拟获取用户登录历史
    let mut history = Vec::new();
    for i in 0..limit.min(5) {
        history.push(LoginRecord {
            login_id: format!("login_{}_{}", user_id, i),
            login_time: chrono::Utc::now() - Duration::hours(i as i64 * 24),
            logout_time: if i > 0 {
                Some(chrono::Utc::now() - Duration::hours((i - 1) as i64 * 24 + 8))
            } else {
                None
            },
            device_id: Some(format!("device_00{}", i + 1)),
            ip_address: Some("192.168.1.100".to_string()),
            user_agent: Some(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            ),
            login_type: "password".to_string(),
            status: "success".to_string(),
        });
    }
    Ok(history)
}

/// 获取用户敏感信息
async fn get_user_sensitive_info(user_id: &str) -> SecurityResult<SensitiveUserInfo> {
    // 模拟获取用户敏感信息
    Ok(SensitiveUserInfo {
        id_card: None,
        bank_account: None,
        emergency_contact: Some(EmergencyContact {
            name: "紧急联系人".to_string(),
            phone: "13900139000".to_string(),
            relationship: "配偶".to_string(),
        }),
        address: Some("北京市朝阳区xxx街道xxx号".to_string()),
        birthday: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        marital_status: Some("已婚".to_string()),
        education: Some("本科".to_string()),
    })
}

/// 检查具体权限
async fn check_specific_permission(
    user_permissions: &[UserPermission],
    permission: &str,
    resource_type: &Option<String>,
    resource_id: &Option<String>,
    context: &Option<String>,
) -> SecurityResult<PermissionCheckResult> {
    // 检查权限是否在用户权限列表中
    let user_permission = user_permissions
        .iter()
        .find(|p| p.permission_code == permission);

    let has_permission = user_permission.is_some()
        && user_permission
            .as_ref()
            .map(|p| p.is_active)
            .unwrap_or(false);

    Ok(PermissionCheckResult {
        has_permission,
        granted_at: user_permission.map(|p| p.granted_at),
        expires_at: user_permission.and_then(|p| p.expires_at),
        granted_by: user_permission.and_then(|p| p.granted_by.clone()),
        restrictions: vec![], // 可以添加权限限制
        cached: false,
    })
}

// ============ 请求和响应模型定义 ============

/// 用户身份验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthRequest {
    /// 用户ID
    pub user_id: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 用户名
    pub username: Option<String>,
    /// 访问令牌
    pub access_token: String,
    /// 验证方法
    pub verify_method: String,
    /// 是否包含个人资料
    pub include_profile: Option<bool>,
    /// 是否包含权限信息
    pub include_permissions: Option<bool>,
    /// 设备ID
    pub device_id: Option<String>,
}

/// 用户身份验证响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub username: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 状态
    pub status: String,
    /// 租户key
    pub tenant_key: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 职位
    pub position: Option<String>,
    /// 是否活跃
    pub is_active: bool,
    /// 是否是管理员
    pub is_admin: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,
    /// 个人资料
    pub profile: Option<UserProfile>,
    /// 权限信息
    pub permissions: Option<Vec<UserPermission>>,
    /// 设备是否受信任
    pub device_trusted: bool,
    /// 验证时间
    pub verified_at: DateTime<Utc>,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
}

/// 用户详细信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetailRequest {
    /// 用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 是否包含敏感信息
    pub include_sensitive: Option<bool>,
    /// 是否包含权限信息
    pub include_permissions: Option<bool>,
    /// 是否包含角色信息
    pub include_roles: Option<bool>,
    /// 是否包含部门信息
    pub include_departments: Option<bool>,
    /// 是否包含登录历史
    pub include_login_history: Option<bool>,
}

/// 用户详细信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetailResponse {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub username: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 状态
    pub status: String,
    /// 租户key
    pub tenant_key: Option<String>,
    /// 工号
    pub employee_number: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 入职时间
    pub join_time: Option<DateTime<Utc>>,
    /// 工位
    pub work_station: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 直属上级
    pub direct_manager: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 部门信息
    pub departments: Option<Vec<DepartmentInfo>>,
    /// 角色信息
    pub roles: Option<Vec<UserRole>>,
    /// 权限信息
    pub permissions: Option<Vec<UserPermission>>,
    /// 是否活跃
    pub is_active: bool,
    /// 是否是管理员
    pub is_admin: bool,
    /// 是否是高级用户
    pub is_senior: Option<bool>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,
    /// 登录历史
    pub login_history: Option<Vec<LoginRecord>>,
    /// 敏感信息
    pub sensitive_info: Option<SensitiveUserInfo>,
    /// 检索时间
    pub retrieved_at: DateTime<Utc>,
}

/// 用户信息更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    /// 用户ID
    pub user_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 昵称
    pub nickname: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 职位
    pub position: Option<String>,
    /// 工号
    pub employee_number: Option<String>,
    /// 工位
    pub work_station: Option<String>,
    /// 手机号可见性
    pub mobile_visible: Option<bool>,
    /// 邮箱可见性
    pub email_visible: Option<bool>,
    /// 性别
    pub gender: Option<String>,
    /// 是否使用gravatar头像
    pub is_avatar_in_gravatar: Option<bool>,
    /// 是否是高级用户
    pub is_senior: Option<bool>,
    /// 角色列表
    pub roles: Option<Vec<String>>,
    /// 自定义属性
    pub custom_attrs: Option<std::collections::HashMap<String, String>>,
}

/// 用户信息更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 更新的字段
    pub updated_fields: Vec<String>,
    /// 更新后的用户信息
    pub updated_info: UserInfo,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 消息
    pub message: String,
}

/// 权限检查请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCheckRequest {
    /// 用户ID（可选，默认为当前用户）
    pub user_id: Option<String>,
    /// 访问令牌
    pub access_token: String,
    /// 权限标识符
    pub permission: String,
    /// 资源类型
    pub resource_type: Option<String>,
    /// 资源ID
    pub resource_id: Option<String>,
    /// 权限检查上下文
    pub context: Option<String>,
}

/// 权限检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    /// 用户ID
    pub user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 资源类型
    pub resource_type: Option<String>,
    /// 资源ID
    pub resource_id: Option<String>,
    /// 权限检查上下文
    pub context: Option<String>,
    /// 是否有权限
    pub has_permission: bool,
    /// 授权时间
    pub granted_at: Option<DateTime<Utc>>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 授权人
    pub granted_by: Option<String>,
    /// 权限限制
    pub restrictions: Vec<String>,
    /// 检查时间
    pub checked_at: DateTime<Utc>,
    /// 是否缓存
    pub cached: bool,
}

// ============ 数据结构定义 ============

/// 用户详细资料
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// 工号
    pub employee_number: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 入职时间
    pub join_time: Option<DateTime<Utc>>,
    /// 工位
    pub work_station: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 直属上级
    pub direct_manager: Option<String>,
    /// 是否是高级用户
    pub is_senior: Option<bool>,
    /// 自定义属性
    pub custom_attributes: std::collections::HashMap<String, String>,
}

/// 用户权限
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermission {
    /// 权限ID
    pub permission_id: String,
    /// 权限代码
    pub permission_code: String,
    /// 权限名称
    pub permission_name: String,
    /// 资源类型
    pub resource_type: String,
    /// 授权时间
    pub granted_at: DateTime<Utc>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 授权人
    pub granted_by: Option<String>,
    /// 是否激活
    pub is_active: bool,
}

/// 用户角色
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    /// 角色ID
    pub role_id: String,
    /// 角色代码
    pub role_code: String,
    /// 角色名称
    pub role_name: String,
    /// 分配时间
    pub assigned_at: DateTime<Utc>,
    /// 分配人
    pub assigned_by: Option<String>,
    /// 是否激活
    pub is_active: bool,
}

/// 部门信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentInfo {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 父部门ID
    pub parent_id: Option<String>,
    /// 部门级别
    pub level: i32,
    /// 成员数量
    pub member_count: i32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 登录记录
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRecord {
    /// 登录ID
    pub login_id: String,
    /// 登录时间
    pub login_time: DateTime<Utc>,
    /// 登出时间
    pub logout_time: Option<DateTime<Utc>>,
    /// 设备ID
    pub device_id: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 登录类型
    pub login_type: String,
    /// 状态
    pub status: String,
}

/// 敏感用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SensitiveUserInfo {
    /// 身份证号
    pub id_card: Option<String>,
    /// 银行账户
    pub bank_account: Option<String>,
    /// 紧急联系人
    pub emergency_contact: Option<EmergencyContact>,
    /// 地址
    pub address: Option<String>,
    /// 生日
    pub birthday: Option<chrono::NaiveDate>,
    /// 婚姻状况
    pub marital_status: Option<String>,
    /// 教育程度
    pub education: Option<String>,
}

/// 紧急联系人
#[derive(Debug, Serialize, Deserialize)]
pub struct EmergencyContact {
    /// 姓名
    pub name: String,
    /// 电话
    pub phone: String,
    /// 关系
    pub relationship: String,
}

/// 权限检查结果
#[derive(Debug)]
struct PermissionCheckResult {
    /// 是否有权限
    has_permission: bool,
    /// 授权时间
    granted_at: Option<DateTime<Utc>>,
    /// 过期时间
    expires_at: Option<DateTime<Utc>>,
    /// 授权人
    granted_by: Option<String>,
    /// 权限限制
    restrictions: Vec<String>,
    /// 是否缓存
    cached: bool,
}

/// 用户信息更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: String,
    /// 更新时间
    pub update_time: chrono::DateTime<chrono::Utc>,
    /// 更新的字段
    pub updated_fields: Vec<String>,
    /// 消息
    pub message: String,
}
