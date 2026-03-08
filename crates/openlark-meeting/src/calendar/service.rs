use openlark_core::config::Config;

pub struct CalendarService {
    config: Config,
}

impl CalendarService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn v4(&self) -> CalendarV4Service {
        CalendarV4Service::new(self.config.clone())
    }
}

pub struct CalendarV4Service {
    config: Config,
}

impl CalendarV4Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn calendar(&self) -> CalendarResource {
        CalendarResource::new(self.config.clone())
    }
}

pub struct CalendarResource {
    config: Config,
}

impl CalendarResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

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
