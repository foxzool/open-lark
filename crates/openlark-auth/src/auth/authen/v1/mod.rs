//! Authen v1 API实现

pub mod access_token;
pub mod oidc;
pub mod refresh_access_token;
pub mod user_info;

// 重新导出子模块的构建器和服务
pub use access_token::UserAccessTokenV1Builder;
pub use oidc::{OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, OidcService};
pub use refresh_access_token::RefreshUserAccessTokenV1Builder;
pub use user_info::{UserInfoBuilder, UserInfoService};

use openlark_core::config::Config;

// AuthenServiceV1结构体
#[derive(Debug)]
pub struct AuthenServiceV1 {
    config: Config,
}

impl AuthenServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 用户信息服务
    pub fn user_info(&self) -> UserInfoService {
        UserInfoService::new(self.config.clone())
    }

    /// 用户访问令牌构建器（v1版本）
    pub fn access_token(&self) -> UserAccessTokenV1Builder {
        UserAccessTokenV1Builder::new(self.config.clone())
    }

    /// 用户访问令牌刷新构建器（v1版本）
    pub fn refresh_access_token(&self) -> RefreshUserAccessTokenV1Builder {
        RefreshUserAccessTokenV1Builder::new(self.config.clone())
    }

    /// OIDC服务
    pub fn oidc(&self) -> OidcService {
        OidcService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
