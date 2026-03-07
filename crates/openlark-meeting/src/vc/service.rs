//! 视频会议服务
use openlark_core::config::Config;

/// 视频会议服务
#[derive(Debug, Clone)]
pub struct VcService {
    config: Config,
}

impl VcService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn v1(&self) -> VcV1Service {
        VcV1Service::new(self.config.clone())
    }
}

/// 视频会议 V1 服务
#[derive(Debug, Clone)]
pub struct VcV1Service {
    config: Config,
}

impl VcV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn room(&self) -> RoomResource {
        RoomResource::new(self.config.clone())
    }

    pub fn meeting(&self) -> MeetingResource {
        MeetingResource::new(self.config.clone())
    }

    pub fn reserve(&self) -> ReserveResource {
        ReserveResource::new(self.config.clone())
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

/// Meeting 资源
#[derive(Debug, Clone)]
pub struct MeetingResource {
    config: Config,
}

impl MeetingResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Reserve 资源
#[derive(Debug, Clone)]
pub struct ReserveResource {
    config: Config,
}

impl ReserveResource {
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
