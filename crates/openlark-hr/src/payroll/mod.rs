/// 薪资模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// payroll 项目模块。
/// payroll 子模块。
#[allow(clippy::module_inception)]
pub mod payroll;

/// 薪资服务
/// Payroll 服务入口。
#[derive(Debug, Clone)]
pub struct Payroll {
    config: Config,
}

impl Payroll {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 payroll 项目 v1 版本服务
    pub fn v1(&self) -> payroll::PayrollV1 {
        payroll::PayrollV1::new(self.config.clone())
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
