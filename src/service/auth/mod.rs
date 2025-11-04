#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Auth服务模块 - 企业级认证API
//!
//! 提供飞书开放平台的核心认证功能，支持：
//! - 应用访问令牌管理
//! - 租户访问令牌管理
//! - 用户信息获取
//! - OIDC认证流程
//!
//! # 版本支持
//!
//! - **v1**: 用户认证相关API
//! - **v3**: 应用和租户令牌管理API
//!
//! # 快速开始
//!
//! ```no_run
//! use open_lark::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取租户访问令牌（自建应用）
//! let token_request = auth::v3::TenantAccessTokenInternalRequest {
//!     app_id: "your_app_id".to_string(),
//!     app_secret: "your_app_secret".to_string(),
//! };
//! let token_response = client.auth.v3.tenant_access_token_internal(&token_request).await?;
//!
//! println!("租户访问令牌: {}", token_response.data.unwrap().tenant_access_token.unwrap());
//! # Ok(())
//! # }
//! ```

use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::core::config::Config;
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
/// 飞书开放平台认证服务的统一入口，提供所有认证相关的API访问能力。
#[derive(Debug, Clone)]
pub struct AuthService {
    pub service: SimpleService,
    /// v1版本API - 用户认证和OIDC流程
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

// Re-export models for convenience
pub use v1::models as v1_models;
pub use v3::models as v3_models;
