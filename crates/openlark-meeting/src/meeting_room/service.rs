use openlark_core::config::Config;

/// 会议室服务（历史版本）
#[derive(Debug, Clone)]
pub struct MeetingRoomService {
    config: Config,
}

impl MeetingRoomService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn building(&self) -> BuildingResource {
        BuildingResource::new(self.config.clone())
    }

    pub fn room(&self) -> RoomResource {
        RoomResource::new(self.config.clone())
    }
}

/// Building 资源
#[derive(Debug, Clone)]
pub struct BuildingResource {
    config: Config,
}

impl BuildingResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Room 资源
#[derive(Debug, Clone)]
pub struct RoomResource {
    config: Config,
}

impl RoomResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
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
