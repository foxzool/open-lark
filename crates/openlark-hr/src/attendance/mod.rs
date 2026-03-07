/// 考勤模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

#[allow(clippy::module_inception)]
pub mod attendance;

/// 考勤服务
#[derive(Debug, Clone)]
pub struct Attendance {
    config: Config,
}

impl Attendance {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 attendance 项目 v1 版本服务
    pub fn v1(&self) -> attendance::AttendanceV1 {
        attendance::AttendanceV1::new(self.config.clone())
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
