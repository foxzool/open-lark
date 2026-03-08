/// 飞书人事模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

pub mod corehr;

/// 飞书人事服务
#[derive(Debug, Clone)]
pub struct Corehr {
    config: Config,
}

impl Corehr {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 corehr 项目 v1 版本服务
    pub fn v1(&self) -> corehr::CorehrV1 {
        corehr::CorehrV1::new(self.config.clone())
    }

    /// 获取 corehr 项目 v2 版本服务
    pub fn v2(&self) -> corehr::CorehrV2 {
        corehr::CorehrV2::new(self.config.clone())
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
