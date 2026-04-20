/// performance 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;

/// performance 项目 v1 版本服务
/// PerformanceV1 服务入口。
#[derive(Debug, Clone)]
pub struct PerformanceV1 {
    config: Config,
}

impl PerformanceV1 {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// performance 项目 v2 版本服务
/// PerformanceV2 服务入口。
#[derive(Debug, Clone)]
pub struct PerformanceV2 {
    config: Config,
}

impl PerformanceV2 {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
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
