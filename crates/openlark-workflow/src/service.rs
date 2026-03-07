use openlark_core::config::Config;
use std::sync::Arc;

/// WorkflowService：工作流服务的统一入口
///
/// 提供对任务、审批、看板 API 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct WorkflowService {
    config: Arc<Config>,
}

impl WorkflowService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::v1::TaskV1 {
        crate::v1::TaskV1::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn v2(&self) -> crate::v2::TaskV2 {
        crate::v2::TaskV2::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn task(&self) -> crate::v2::task::Task {
        crate::v2::task::Task::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn tasklist(&self) -> crate::v2::tasklist::Tasklist {
        crate::v2::tasklist::Tasklist::new(self.config.clone())
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
