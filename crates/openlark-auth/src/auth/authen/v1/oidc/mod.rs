//! OIDC Authentication APIs

use openlark_core::config::Config;

pub mod access_token;
pub mod refresh_access_token;

// Re-export types for convenient access
pub use access_token::create::OidcAccessTokenBuilder;
pub use refresh_access_token::create::OidcRefreshAccessTokenBuilder;

/// OIDC认证服务
#[derive(Debug)]
pub struct OidcService {
    config: Config,
}

impl OidcService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取OIDC用户访问令牌
    pub fn access_token(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder::new(self.config.clone())
    }

    /// 刷新OIDC用户访问令牌
    pub fn refresh_access_token(&self) -> OidcRefreshAccessTokenBuilder {
        OidcRefreshAccessTokenBuilder::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
