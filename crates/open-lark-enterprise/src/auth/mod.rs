//! Auth服务模块
//!
//! 提供飞书开放平台的认证相关API，包括：
//! - 应用访问令牌获取（自建应用和商店应用）
//! - 租户访问令牌获取
//! - 用户信息获取
//! - OIDC认证流程
//! - 应用票据重发
//!
//! # 版本支持
//!
//! - **v1**: 用户认证和OIDC流程
//! - **v3**: 应用和租户令牌管理
//!
//! # 使用示例
//!
//! ```no_run
//! use open_lark::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建客户端
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取租户访问令牌（自建应用）
//! let token_request = TenantAccessTokenInternalRequest {
//!     app_id: "your_app_id".to_string(),
//!     app_secret: "your_app_secret".to_string(),
//! };
//! let token_response = client.auth.v3.tenant_access_token_internal(&token_request).await?;
//!
//! // 获取用户信息
//! let user_info = client.auth.v1.user_info().await?;
//! # Ok(())
//! # }
//! ```

use open_lark_core::core::api_resp::{ApiResponseTrait, ResponseFormat};
use open_lark_core::core::config::Config;
use serde::{Deserialize, Serialize};

// 声明版本模块
pub mod v1;
pub mod v3;

/// 简化的服务结构体
#[derive(Debug, Clone)]
pub struct SimpleService {
    pub config: Config,
}

impl SimpleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;

impl ApiResponseTrait for SimpleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Auth服务
///
/// 提供飞书开放平台的所有认证相关功能，包括令牌管理、用户认证等。
#[derive(Debug, Clone)]
pub struct AuthService {
    pub service: SimpleService,
    /// v1版本API - 用户认证和OIDC
    pub v1: v1::AuthServiceV1,
    /// v3版本API - 应用和租户令牌管理
    pub v3: v3::AuthServiceV3,
}

impl AuthService {
    /// 创建新的Auth服务实例
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config.clone()),
            v1: v1::AuthServiceV1::new(config.clone()),
            v3: v3::AuthServiceV3::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = AuthService;
pub type ResponseType = SimpleResponse;
