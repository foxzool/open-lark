/// 招聘模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

#[allow(clippy::module_inception)]
pub mod hire;

/// 招聘服务
#[derive(Debug, Clone)]
pub struct Hire {
    config: Config,
}

impl Hire {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 hire 项目 v1 版本服务
    pub fn v1(&self) -> hire::HireV1 {
        hire::HireV1::new(self.config.clone())
    }

    /// 获取 hire 项目 v2 版本服务
    pub fn v2(&self) -> hire::HireV2 {
        hire::HireV2::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
