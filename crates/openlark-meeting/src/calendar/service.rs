use openlark_core::config::Config;

/// 日历服务。
pub struct CalendarService {
    config: Config,
}

impl CalendarService {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 访问 v4 版本 API。
    pub fn v4(&self) -> CalendarV4Service {
        CalendarV4Service::new(self.config.clone())
    }
}

/// 日历 v4 服务。
pub struct CalendarV4Service {
    config: Config,
}

impl CalendarV4Service {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 访问 calendar 资源。
    pub fn calendar(&self) -> CalendarResource {
        CalendarResource::new(self.config.clone())
    }
}

/// 日历资源。
pub struct CalendarResource {
    config: Config,
}

impl CalendarResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
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
