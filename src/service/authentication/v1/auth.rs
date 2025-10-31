//! 认证服务 V1 版本
//!
//! 提供完整的飞书开放平台认证功能，包括：
//! - 用户信息获取和管理
//! - 应用访问令牌（App Access Token）管理
//! - 租户访问令牌（Tenant Access Token）管理
//! - 应用票据（App Ticket）管理
//!
//! # 功能特性
//!
//! ## 用户信息服务 (UserInfoService)
//! - 获取当前登录用户的详细信息
//! - 支持多种用户ID类型（open_id、user_id、union_id）
//! - 返回用户完整资料，包括头像、联系方式等
//!
//! ## 应用访问令牌服务 (AppAccessTokenService)
//! - 商店应用获取App Access Token
//! - 自建应用获取App Access Token
//! - 支持令牌自动刷新机制
//! - 提供令牌有效期管理
//!
//! ## 租户访问令牌服务 (TenantAccessTokenService)
//! - 商店应用获取Tenant Access Token
//! - 自建应用获取Tenant Access Token
//! - 支持企业级多租户管理
//! - 令牌安全性和有效性验证
//!
//! ## 应用票据服务 (AppTicketService)
//! - 重新获取App Ticket
//! - 支持事件推送凭证管理
//! - 提供回调地址配置
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::authentication::v1::auth::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取用户信息
//!     let user_info = client.authen.v1.user_info
//!         .get_user_info_builder()
//!         .user_access_token("user_access_token")
//!         .execute(&client.authen.v1.user_info)
//!         .await?;
//!
//!     println!("用户姓名: {}", user_info.data.name);
//!     println!("用户邮箱: {:?}", user_info.data.email);
//!
//!     // 获取应用访问令牌
//!     let app_token_response = client.authen.v1.app_access_token
//!         .get_app_token_builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .execute(&client.authen.v1.app_access_token)
//!         .await?;
//!
//!     println!("应用访问令牌: {}", app_token_response.data.app_access_token);
//!
//!     Ok(())
//! }
//! ```

use open_lark_core::core::trait_system::ExecutableBuilder;
use crate::core::{
    api_req::ApiRequest,
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    endpoints::auth::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ==================== 用户信息服务 ====================

/// 用户信息服务
///
/// 提供获取当前登录用户详细信息的功能。
/// 支持获取用户的基本资料、头像、联系方式等完整信息。
///
/// # 主要功能
///
/// - 获取登录用户的完整资料
/// - 支持多种认证令牌类型
/// - 提供用户头像的多尺寸版本
/// - 返回用户的联系信息和工作信息
///
/// # 使用场景
///
/// - 用户登录后获取个人信息
/// - 构建用户个人资料页面
/// - 用户身份验证和权限管理
/// - 个性化内容推荐
#[derive(Debug, Clone)]
pub struct UserInfoService {
    /// SDK配置信息
    pub config: Config,
}

impl UserInfoService {
    /// 创建用户信息服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息，包含应用ID、密钥等
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::UserInfoService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = UserInfoService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户信息
    ///
    /// 根据用户访问令牌获取当前登录用户的详细信息。
    /// 返回用户的基本资料、头像链接、联系方式等完整信息。
    ///
    /// # API文档
    ///
    /// 获取用户信息的API接口，支持通过用户访问令牌获取用户详细资料。
    ///
    /// # 参数
    ///
    /// * `user_access_token` - 用户访问令牌，用于识别用户身份
    ///
    /// # 返回值
    ///
    /// 返回包含用户完整信息的响应数据
    ///
    /// # 错误处理
    ///
    /// - 令牌无效或过期时返回认证错误
    /// - 用户不存在时返回用户未找到错误
    /// - 网络问题时返回连接错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let response = client.authen.v1.user_info
    ///     .get("user_access_token_here").await?;
    /// println!("用户姓名: {}", response.data.name);
    /// println!("用户邮箱: {:?}", response.data.email);
    /// ```
    pub async fn get(
        &self,
        user_access_token: impl ToString,
    ) -> SDKResult<BaseResponse<UserInfo>> {
        let api_req = ApiRequest {
            api_path: AUTHEN_V1_USER_INFO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let option = RequestOption::builder()
            .user_access_token(user_access_token)
            .build();

        let api_resp: BaseResponse<UserInfo> =
            Transport::request(api_req, &self.config, Some(option)).await?;
        Ok(api_resp)
    }

    /// 获取用户信息构建器
    ///
    /// 提供流式API来构建获取用户信息的请求参数。
    /// 支持链式调用，方便配置用户访问令牌等参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let builder = GetUserInfoBuilder::new()
    ///     .user_access_token("user_access_token_here");
    ///
    /// let response = builder.execute(&client.authen.v1.user_info).await?;
    /// ```
    pub fn get_user_info_builder(&self) -> GetUserInfoBuilder {
        GetUserInfoBuilder::new()
    }
}

// ==================== 应用访问令牌服务 ====================

/// 应用访问令牌服务
///
/// 提供应用级访问令牌的获取和管理功能。
/// 支持商店应用和自建应用的不同认证流程。
///
/// # 主要功能
///
/// - 商店应用获取App Access Token
/// - 自建应用获取App Access Token
/// - 令牌有效期管理
/// - 自动令牌刷新支持
///
/// # 使用场景
///
/// - 应用启动时获取访问令牌
/// - 调用不需要用户授权的API接口
/// - 后台服务间的API调用
/// - 应用服务器端的API访问
#[derive(Debug, Clone)]
pub struct AppAccessTokenService {
    /// SDK配置信息
    pub config: Config,
}

impl AppAccessTokenService {
    /// 创建应用访问令牌服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::AppAccessTokenService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = AppAccessTokenService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取商店应用访问令牌
    ///
    /// 应用商店应用通过app_id和app_secret获取应用访问令牌。
    /// App Access Token用于访问不需要用户授权的API接口。
    ///
    /// # API文档
    ///
    /// 商店应用获取App Access Token的标准接口。
    ///
    /// # 参数
    ///
    /// * `request` - 获取令牌的请求参数，包含应用凭证信息
    ///
    /// # 返回值
    ///
    /// 返回应用访问令牌和相关信息
    ///
    /// # 令牌使用
    ///
    /// - 令牌有效期通常为2小时
    /// - 建议在令牌过期前15分钟刷新
    /// - 支持通过refresh_token自动续期
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let request = GetAppAccessTokenRequest {
    ///     app_id: "your_app_id".to_string(),
    ///     app_secret: "your_app_secret".to_string(),
    ///     app_type: Some("marketplace".to_string()),
    /// };
    ///
    /// let response = client.authen.v1.app_access_token
    ///     .get(&request).await?;
    /// println!("应用访问令牌: {}", response.data.app_access_token);
    /// ```
    pub async fn get(
        &self,
        request: &GetAppAccessTokenRequest,
    ) -> SDKResult<BaseResponse<AppAccessTokenResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_ACCESS_TOKEN.to_string(),
            supported_access_token_types: vec![],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        Ok(resp)
    }

    /// 获取自建应用访问令牌
    ///
    /// 企业自建应用通过app_id和app_secret获取应用访问令牌。
    /// App Access Token用于访问不需要用户授权的API接口。
    ///
    /// # API文档
    ///
    /// 自建应用获取App Access Token的标准接口。
    ///
    /// # 参数
    ///
    /// * `request` - 获取令牌的请求参数，包含应用凭证信息
    ///
    /// # 返回值
    ///
    /// 返回应用访问令牌和相关信息
    ///
    /// # 企业级特性
    ///
    /// - 支持企业内部应用认证
    /// - 更高的API调用频率限制
    /// - 企业级安全策略支持
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let request = GetAppAccessTokenInternalRequest {
    ///     app_id: "your_app_id".to_string(),
    ///     app_secret: "your_app_secret".to_string(),
    /// };
    ///
    /// let response = client.authen.v1.app_access_token
    ///     .get_internal(&request).await?;
    /// println!("应用访问令牌: {}", response.data.app_access_token);
    /// ```
    pub async fn get_internal(
        &self,
        request: &GetAppAccessTokenInternalRequest,
    ) -> SDKResult<BaseResponse<AppAccessTokenResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_ACCESS_TOKEN_INTERNAL.to_string(),
            supported_access_token_types: vec![],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        Ok(resp)
    }

    /// 获取商店应用访问令牌构建器
    pub fn get_app_token_builder(&self) -> GetAppAccessTokenBuilder {
        GetAppAccessTokenBuilder::new()
    }

    /// 获取自建应用访问令牌构建器
    pub fn get_app_token_internal_builder(&self) -> GetAppAccessTokenInternalBuilder {
        GetAppAccessTokenInternalBuilder::new()
    }
}

// ==================== 租户访问令牌服务 ====================

/// 租户访问令牌服务
///
/// 提供租户级访问令牌的获取和管理功能。
/// 支持商店应用和自建应用的不同认证流程。
///
/// # 主要功能
///
/// - 商店应用获取Tenant Access Token
/// - 自建应用获取Tenant Access Token
/// - 多租户管理支持
/// - 企业级权限控制
///
/// # 使用场景
///
/// - 访问企业级数据和API
/// - 企业管理后台操作
/// - 多租户SaaS应用开发
/// - 企业集成和定制开发
#[derive(Debug, Clone)]
pub struct TenantAccessTokenService {
    /// SDK配置信息
    pub config: Config,
}

impl TenantAccessTokenService {
    /// 创建租户访问令牌服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取商店应用租户访问令牌
    ///
    /// 应用商店应用获取租户访问令牌，用于访问特定企业的资源和数据。
    /// 需要企业管理员的授权和配置。
    ///
    /// # API文档
    ///
    /// 商店应用获取Tenant Access Token的标准接口。
    ///
    /// # 参数
    ///
    /// * `request` - 获取租户令牌的请求参数
    ///
    /// # 返回值
    ///
    /// 返回租户访问令牌和相关信息
    ///
    /// # 权限要求
    ///
    /// - 需要企业管理员授权
    /// - 应用必须安装到目标企业
    /// - 需要配置相应的权限范围
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let request = GetTenantAccessTokenRequest {
    ///     app_id: "your_app_id".to_string(),
    ///     app_secret: "your_app_secret".to_string(),
    ///     tenant_key: Some("tenant_key".to_string()),
    /// };
    ///
    /// let response = client.authen.v1.tenant_access_token
    ///     .get(&request).await?;
    /// println!("租户访问令牌: {}", response.data.tenant_access_token);
    /// ```
    pub async fn get(
        &self,
        request: &GetTenantAccessTokenRequest,
    ) -> SDKResult<BaseResponse<TenantAccessTokenResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_TENANT_ACCESS_TOKEN.to_string(),
            supported_access_token_types: vec![],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        Ok(resp)
    }

    /// 获取自建应用租户访问令牌
    ///
    /// 企业自建应用获取租户访问令牌，用于访问特定企业的资源和数据。
    /// 需要企业管理员的授权和配置。
    ///
    /// # API文档
    ///
    /// 自建应用获取Tenant Access Token的标准接口。
    ///
    /// # 参数
    ///
    /// * `request` - 获取租户令牌的请求参数
    ///
    /// # 返回值
    ///
    /// 返回租户访问令牌和相关信息
    ///
    /// # 企业级特性
    ///
    /// - 支持企业内部应用认证
    /// - 更高的API调用频率限制
    /// - 企业级安全策略支持
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let request = GetTenantAccessTokenInternalRequest {
    ///     app_id: "your_app_id".to_string(),
    ///     app_secret: "your_app_secret".to_string(),
    ///     tenant_key: Some("tenant_key".to_string()),
    /// };
    ///
    /// let response = client.authen.v1.tenant_access_token
    ///     .get_internal(&request).await?;
    /// println!("租户访问令牌: {}", response.data.tenant_access_token);
    /// ```
    pub async fn get_internal(
        &self,
        request: &GetTenantAccessTokenInternalRequest,
    ) -> SDKResult<BaseResponse<TenantAccessTokenResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_TENANT_ACCESS_TOKEN_INTERNAL.to_string(),
            supported_access_token_types: vec![],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        Ok(resp)
    }

    /// 获取商店应用租户访问令牌构建器
    pub fn get_tenant_token_builder(&self) -> GetTenantAccessTokenBuilder {
        GetTenantAccessTokenBuilder::new()
    }

    /// 获取自建应用租户访问令牌构建器
    pub fn get_tenant_token_internal_builder(&self) -> GetTenantAccessTokenInternalBuilder {
        GetTenantAccessTokenInternalBuilder::new()
    }
}

// ==================== 应用票据服务 ====================

/// 应用票据服务
///
/// 提供App Ticket的管理功能，用于事件推送凭证管理。
/// 支持重新获取App Ticket，确保事件推送的正常工作。
///
/// # 主要功能
///
/// - 重新获取App Ticket
/// - 事件推送凭证管理
/// - 回调地址配置
/// - 票据状态监控
///
/// # 使用场景
///
/// - 应用事件订阅和推送
/// - Webhook事件处理
/// - 实时数据同步
/// - 应用状态监控
#[derive(Debug, Clone)]
pub struct AppTicketService {
    /// SDK配置信息
    pub config: Config,
}

impl AppTicketService {
    /// 创建应用票据服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 重新获取App Ticket
    ///
    /// 触发飞书重新推送app_ticket，用于解决ticket丢失或失效的问题。
    /// App Ticket是应用接收事件推送的重要凭证。
    ///
    /// # API文档
    ///
    /// 重新获取App Ticket的标准接口。
    ///
    /// # 参数
    ///
    /// * `request` - 重新获取票据的请求参数
    ///
    /// # 返回值
    ///
    /// 返回重新发送操作的结果和新的App Ticket
    ///
    /// # 使用时机
    ///
    /// - App Ticket丢失或失效时
    /// - 事件推送中断时
    /// - 应用重新部署后
    /// - 票据即将过期时
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::auth::*;
    ///
    /// let request = ResendAppTicketRequest {
    ///     app_id: "your_app_id".to_string(),
    ///     app_secret: "your_app_secret".to_string(),
    ///     callback_address: Some("https://your-domain.com/callback".to_string()),
    /// };
    ///
    /// let response = client.authen.v1.app_ticket
    ///     .resend(&request).await?;
    /// println!("App Ticket重新发送状态: {:?}", response.data.status);
    /// ```
    pub async fn resend(
        &self,
        request: &ResendAppTicketRequest,
    ) -> SDKResult<BaseResponse<ResendAppTicketResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_TICKET_RESEND.to_string(),
            supported_access_token_types: vec![],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        Ok(resp)
    }

    /// 重新获取App Ticket构建器
    pub fn resend_app_ticket_builder(&self) -> ResendAppTicketBuilder {
        ResendAppTicketBuilder::new()
    }
}

// ==================== Builder模式实现 ====================

// 用户信息构建器
/// 获取用户信息构建器
#[derive(Debug, Clone)]
pub struct GetUserInfoBuilder {
    user_access_token: Option<String>,
}

impl GetUserInfoBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            user_access_token: None,
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl ToString) -> Self {
        self.user_access_token = Some(user_access_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> String {
        self.user_access_token.unwrap_or_default()
    }
}

impl Default for GetUserInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait for UserInfo
crate::impl_executable_builder!(
    GetUserInfoBuilder,
    UserInfoService,
    String,
    BaseResponse<UserInfo>,
    get
);

// 应用访问令牌构建器
/// 商店应用获取App Access Token构建器
#[derive(Debug, Clone)]
pub struct GetAppAccessTokenBuilder {
    request: GetAppAccessTokenRequest,
}

impl GetAppAccessTokenBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetAppAccessTokenRequest::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl ToString) -> Self {
        self.request.app_id = app_id.to_string();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl ToString) -> Self {
        self.request.app_secret = app_secret.to_string();
        self
    }

    /// 设置应用类型
    pub fn app_type(mut self, app_type: impl ToString) -> Self {
        self.request.app_type = Some(app_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetAppAccessTokenRequest {
        self.request
    }
}

impl Default for GetAppAccessTokenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

crate::impl_executable_builder!(
    GetAppAccessTokenBuilder,
    AppAccessTokenService,
    GetAppAccessTokenRequest,
    BaseResponse<AppAccessTokenResponse>,
    get
);

/// 自建应用获取App Access Token构建器
#[derive(Debug, Clone)]
pub struct GetAppAccessTokenInternalBuilder {
    request: GetAppAccessTokenInternalRequest,
}

impl GetAppAccessTokenInternalBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetAppAccessTokenInternalRequest::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl ToString) -> Self {
        self.request.app_id = app_id.to_string();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl ToString) -> Self {
        self.request.app_secret = app_secret.to_string();
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetAppAccessTokenInternalRequest {
        self.request
    }
}

impl Default for GetAppAccessTokenInternalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

crate::impl_executable_builder!(
    GetAppAccessTokenInternalBuilder,
    AppAccessTokenService,
    GetAppAccessTokenInternalRequest,
    BaseResponse<AppAccessTokenResponse>,
    get_internal
);

// 租户访问令牌构建器
/// 商店应用获取Tenant Access Token构建器
#[derive(Debug, Clone)]
pub struct GetTenantAccessTokenBuilder {
    request: GetTenantAccessTokenRequest,
}

impl GetTenantAccessTokenBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetTenantAccessTokenRequest::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl ToString) -> Self {
        self.request.app_id = app_id.to_string();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl ToString) -> Self {
        self.request.app_secret = app_secret.to_string();
        self
    }

    /// 设置企业标识
    pub fn tenant_key(mut self, tenant_key: impl ToString) -> Self {
        self.request.tenant_key = Some(tenant_key.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetTenantAccessTokenRequest {
        self.request
    }
}

impl Default for GetTenantAccessTokenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

crate::impl_executable_builder!(
    GetTenantAccessTokenBuilder,
    TenantAccessTokenService,
    GetTenantAccessTokenRequest,
    BaseResponse<TenantAccessTokenResponse>,
    get
);

/// 自建应用获取Tenant Access Token构建器
#[derive(Debug, Clone)]
pub struct GetTenantAccessTokenInternalBuilder {
    request: GetTenantAccessTokenInternalRequest,
}

impl GetTenantAccessTokenInternalBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetTenantAccessTokenInternalRequest::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl ToString) -> Self {
        self.request.app_id = app_id.to_string();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl ToString) -> Self {
        self.request.app_secret = app_secret.to_string();
        self
    }

    /// 设置企业标识
    pub fn tenant_key(mut self, tenant_key: impl ToString) -> Self {
        self.request.tenant_key = Some(tenant_key.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetTenantAccessTokenInternalRequest {
        self.request
    }
}

impl Default for GetTenantAccessTokenInternalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

crate::impl_executable_builder!(
    GetTenantAccessTokenInternalBuilder,
    TenantAccessTokenService,
    GetTenantAccessTokenInternalRequest,
    BaseResponse<TenantAccessTokenResponse>,
    get_internal
);

// 应用票据构建器
/// 重新获取App Ticket构建器
#[derive(Debug, Clone)]
pub struct ResendAppTicketBuilder {
    request: ResendAppTicketRequest,
}

impl ResendAppTicketBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: ResendAppTicketRequest::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl ToString) -> Self {
        self.request.app_id = app_id.to_string();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl ToString) -> Self {
        self.request.app_secret = app_secret.to_string();
        self
    }

    /// 设置回调地址
    pub fn callback_address(mut self, callback_address: impl ToString) -> Self {
        self.request.callback_address = Some(callback_address.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> ResendAppTicketRequest {
        self.request
    }
}

impl Default for ResendAppTicketBuilder {
    fn default() -> Self {
        Self::new()
    }
}

crate::impl_executable_builder!(
    ResendAppTicketBuilder,
    AppTicketService,
    ResendAppTicketRequest,
    BaseResponse<ResendAppTicketResponse>,
    resend
);

// ==================== 数据模型 ====================

/// 登录用户信息
///
/// 包含用户的详细资料信息，包括基本信息、头像、联系方式等。
/// 支持多种用户标识符类型，便于不同场景下的用户识别。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户姓名
    pub name: String,

    /// 用户英文名称
    pub en_name: String,

    /// 用户头像URL
    pub avatar_url: String,

    /// 用户头像 72x72
    pub avatar_thumb: String,

    /// 用户头像 240x240
    pub avatar_middle: String,

    /// 用户头像 640x640
    pub avatar_big: String,

    /// 用户在应用内的唯一标识
    pub open_id: String,

    /// 用户对ISV的唯一标识，对于同一个ISV，用户在其名下所有应用的union_id相同
    pub union_id: String,

    /// 用户邮箱（可选）
    pub email: Option<String>,

    /// 企业邮箱（可选），请先确保已在管理后台启用飞书邮箱服务
    pub enterprise_email: Option<String>,

    /// 用户 user_id
    pub user_id: String,

    /// 用户手机号（可选）
    pub mobile: Option<String>,

    /// 当前企业标识
    pub tenant_key: String,

    /// 用户工号
    pub employee_no: String,
}

// 商店应用获取App Access Token请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 应用类型，app_access_token接口可传递app_type为self_build或marketplace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
}

impl Default for GetAppAccessTokenRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            app_type: None,
        }
    }
}

/// 自建应用获取App Access Token请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

impl Default for GetAppAccessTokenInternalRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
        }
    }
}

/// 商店应用获取Tenant Access Token请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTenantAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 企业标识（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

impl Default for GetTenantAccessTokenRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            tenant_key: None,
        }
    }
}

/// 自建应用获取Tenant Access Token请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTenantAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 企业标识（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

impl Default for GetTenantAccessTokenInternalRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            tenant_key: None,
        }
    }
}

/// 重新获取App Ticket请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendAppTicketRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 接收ticket的回调地址（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_address: Option<String>,
}

impl Default for ResendAppTicketRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            callback_address: None,
        }
    }
}

/// App Access Token响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenResponse {
    /// 应用访问令牌
    pub app_access_token: String,
    /// 令牌类型，目前固定为"bearer"
    pub token_type: String,
    /// 令牌有效期，秒数
    #[serde(rename = "expire")]
    pub expires_in: i64,
    /// 刷新令牌，用于获取新的app_access_token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// 刷新令牌有效期，秒数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
}

/// Tenant Access Token响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenResponse {
    /// 租户访问令牌
    pub tenant_access_token: String,
    /// 令牌类型，目前固定为"bearer"
    pub token_type: String,
    /// 令牌有效期，秒数
    #[serde(rename = "expire")]
    pub expires_in: i64,
    /// 刷新令牌，用于获取新的tenant_access_token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// 刷新令牌有效期，秒数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
}

/// 重新获取App Ticket响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendAppTicketResponse {
    /// App ticket，用于接收事件推送（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ticket: Option<String>,
    /// 重新发送的状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// ==================== 测试模块 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};
    use std::sync::Arc;

    #[test]
    fn test_user_info_deserialization() {
        let json_str = r#"{
            "name": "张三",
            "en_name": "zhangsan",
            "avatar_url": "www.feishu.cn/avatar/icon",
            "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
            "avatar_middle": "www.feishu.cn/avatar/icon_middle",
            "avatar_big": "www.feishu.cn/avatar/icon_big",
            "open_id": "ou-caecc734c2e3328a62489fe0648c4b98779515d3",
            "union_id": "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj",
            "email": "zhangsan@feishu.cn",
            "enterprise_email": "demo@mail.com",
            "user_id": "5d9bdxxx",
            "mobile": "+86130002883xx",
            "tenant_key": "736588c92lxf175d",
            "employee_no": "111222333"
        }"#;

        let user_info: UserInfo = serde_json::from_str(json_str)
            .expect("Failed to parse test user info JSON");

        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.en_name, "zhangsan");
        assert_eq!(user_info.avatar_url, "www.feishu.cn/avatar/icon");
        assert_eq!(user_info.open_id, "ou-caecc734c2e3328a62489fe0648c4b98779515d3");
        assert_eq!(user_info.union_id, "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj");
        assert_eq!(user_info.email, Some("zhangsan@feishu.cn".to_string()));
        assert_eq!(user_info.enterprise_email, Some("demo@mail.com".to_string()));
        assert_eq!(user_info.user_id, "5d9bdxxx");
        assert_eq!(user_info.mobile, Some("+86130002883xx".to_string()));
        assert_eq!(user_info.tenant_key, "736588c92lxf175d");
        assert_eq!(user_info.employee_no, "111222333");
    }

    #[test]
    fn test_user_info_optional_fields() {
        let json_str = r#"{
            "name": "testuser",
            "en_name": "testuser",
            "avatar_url": "www.feishu.cn/avatar/icon",
            "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
            "avatar_middle": "www.feishu.cn/avatar/icon_middle",
            "avatar_big": "www.feishu.cn/avatar/icon_big",
            "open_id": "ou-test123456789",
            "union_id": "on-test123456789",
            "user_id": "test123",
            "tenant_key": "test_tenant",
            "employee_no": "EMP001"
        }"#;

        let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "testuser");
        assert_eq!(user_info.user_id, "test123");
        assert!(user_info.email.is_none());
        assert!(user_info.enterprise_email.is_none());
        assert!(user_info.mobile.is_none());
    }

    #[test]
    fn test_service_creation() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        let user_service = UserInfoService::new(config.clone());
        let app_token_service = AppAccessTokenService::new(config.clone());
        let tenant_token_service = TenantAccessTokenService::new(config.clone());
        let app_ticket_service = AppTicketService::new(config);

        // 验证服务创建成功
        assert_eq!(user_service.config.app_id, "test_app");
        assert_eq!(app_token_service.config.app_id, "test_app");
        assert_eq!(tenant_token_service.config.app_id, "test_app");
        assert_eq!(app_ticket_service.config.app_id, "test_app");
    }

    #[test]
    fn test_builder_default_implementations() {
        // 测试所有Builder的默认实现
        let user_info_builder = GetUserInfoBuilder::default();
        assert!(user_info_builder.user_access_token.is_none());

        let app_token_builder = GetAppAccessTokenBuilder::default();
        assert_eq!(app_token_builder.request.app_id, "");

        let app_token_internal_builder = GetAppAccessTokenInternalBuilder::default();
        assert_eq!(app_token_internal_builder.request.app_id, "");

        let tenant_token_builder = GetTenantAccessTokenBuilder::default();
        assert_eq!(tenant_token_builder.request.app_id, "");

        let tenant_token_internal_builder = GetTenantAccessTokenInternalBuilder::default();
        assert_eq!(tenant_token_internal_builder.request.app_id, "");

        let app_ticket_builder = ResendAppTicketBuilder::default();
        assert_eq!(app_ticket_builder.request.app_id, "");
    }

    #[test]
    fn test_request_default_implementations() {
        // 测试所有请求结构的默认实现
        let app_token_req = GetAppAccessTokenRequest::default();
        assert_eq!(app_token_req.app_id, "");
        assert_eq!(app_token_req.app_secret, "");
        assert!(app_token_req.app_type.is_none());

        let app_token_internal_req = GetAppAccessTokenInternalRequest::default();
        assert_eq!(app_token_internal_req.app_id, "");
        assert_eq!(app_token_internal_req.app_secret, "");

        let tenant_token_req = GetTenantAccessTokenRequest::default();
        assert_eq!(tenant_token_req.app_id, "");
        assert_eq!(tenant_token_req.app_secret, "");
        assert!(tenant_token_req.tenant_key.is_none());

        let tenant_token_internal_req = GetTenantAccessTokenInternalRequest::default();
        assert_eq!(tenant_token_internal_req.app_id, "");
        assert_eq!(tenant_token_internal_req.app_secret, "");
        assert!(tenant_token_internal_req.tenant_key.is_none());

        let app_ticket_req = ResendAppTicketRequest::default();
        assert_eq!(app_ticket_req.app_id, "");
        assert_eq!(app_ticket_req.app_secret, "");
        assert!(app_ticket_req.callback_address.is_none());
    }

    #[test]
    fn test_builder_chaining() {
        // 测试构建器的链式调用
        let app_token_builder = GetAppAccessTokenBuilder::new()
            .app_id("test_app")
            .app_secret("test_secret")
            .app_type("marketplace");

        let request = app_token_builder.build();
        assert_eq!(request.app_id, "test_app");
        assert_eq!(request.app_secret, "test_secret");
        assert_eq!(request.app_type, Some("marketplace".to_string()));

        let tenant_token_builder = GetTenantAccessTokenBuilder::new()
            .app_id("test_app")
            .app_secret("test_secret")
            .tenant_key("test_tenant");

        let request = tenant_token_builder.build();
        assert_eq!(request.app_id, "test_app");
        assert_eq!(request.app_secret, "test_secret");
        assert_eq!(request.tenant_key, Some("test_tenant".to_string()));
    }

    #[test]
    fn test_user_info_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = UserInfoService::new(config1);
        let service2 = UserInfoService::new(config2);

        // 验证服务配置的独立性
        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_app_token_response_structure() {
        let json_str = r#"{
            "app_access_token": "test_token",
            "token_type": "bearer",
            "expire": 7200,
            "refresh_token": "refresh_token",
            "refresh_expires_in": 604800
        }"#;

        let response: AppAccessTokenResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.app_access_token, "test_token");
        assert_eq!(response.token_type, "bearer");
        assert_eq!(response.expires_in, 7200);
        assert_eq!(response.refresh_token, Some("refresh_token".to_string()));
        assert_eq!(response.refresh_expires_in, Some(604800));
    }

    #[test]
    fn test_tenant_token_response_structure() {
        let json_str = r#"{
            "tenant_access_token": "test_tenant_token",
            "token_type": "bearer",
            "expire": 7200
        }"#;

        let response: TenantAccessTokenResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.tenant_access_token, "test_tenant_token");
        assert_eq!(response.token_type, "bearer");
        assert_eq!(response.expires_in, 7200);
        assert!(response.refresh_token.is_none());
        assert!(response.refresh_expires_in.is_none());
    }

    #[test]
    fn test_app_ticket_response_structure() {
        let json_str = r#"{
            "app_ticket": "test_ticket",
            "status": "success"
        }"#;

        let response: ResendAppTicketResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.app_ticket, Some("test_ticket".to_string()));
        assert_eq!(response.status, Some("success".to_string()));
    }

    #[test]
    fn test_service_thread_safety() {
        use std::thread;

        let config = Config::builder()
            .app_id("thread_test_app")
            .app_secret("thread_test_secret")
            .build();

        let service = Arc::new(UserInfoService::new(config));
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let service_clone = Arc::clone(&service);
                thread::spawn(move || {
                    format!("thread_{}_config_app_id: {}", i, service_clone.config.app_id)
                })
            })
            .collect();

        // 所有线程应该能够安全地访问服务
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.contains("thread_test_app"));
        }
    }
}