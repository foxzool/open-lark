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
