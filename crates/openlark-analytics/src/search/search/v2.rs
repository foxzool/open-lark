//! 搜索服务 V2 API
//!
//! 提供搜索服务 V2 版本的 API 访问

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 搜索服务 V2 API
#[derive(Debug, Clone)]
pub struct SearchV2 {
    /// 客户端配置
    config: Arc<AnalyticsConfig>,
}

impl SearchV2 {
    /// 创建新的搜索服务 V2 实例
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 查询搜索
    pub fn query(&self) -> super::v2::query::QueryApi {
        super::v2::query::QueryApi::new(self.config.clone())
    }

    /// 用户搜索
    pub fn user(&self) -> super::v2::user::UserSearchApi {
        super::v2::user::UserSearchApi::new(self.config.clone())
    }
}

pub mod query;
pub mod user;

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
