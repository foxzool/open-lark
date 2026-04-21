use openlark_core::config::Config;

/// 会议室服务（历史版本）
#[derive(Debug, Clone)]
pub struct MeetingRoomService {
    config: Config,
}

impl MeetingRoomService {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 访问 building 资源。
    pub fn building(&self) -> BuildingResource {
        BuildingResource::new(self.config.clone())
    }

    /// 访问 room 资源。
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
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
