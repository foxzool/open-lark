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

    pub fn create(&self) -> crate::vc::vc::v1::room::CreateRoomRequestBuilder {
        crate::vc::vc::v1::room::CreateRoomRequestBuilder::new(self.config.clone())
    }

    pub fn get(&self) -> crate::vc::vc::v1::room::GetRoomRequestBuilder {
        crate::vc::vc::v1::room::GetRoomRequestBuilder::new(self.config.clone())
    }

    pub fn delete(&self) -> crate::vc::vc::v1::room::DeleteRoomRequestBuilder {
        crate::vc::vc::v1::room::DeleteRoomRequestBuilder::new(self.config.clone())
    }

    pub fn list(&self) -> crate::vc::vc::v1::room::ListRoomRequestBuilder {
        crate::vc::vc::v1::room::ListRoomRequestBuilder::new(self.config.clone())
    }

    pub fn mget(&self) -> crate::vc::vc::v1::room::MgetRoomRequestBuilder {
        crate::vc::vc::v1::room::MgetRoomRequestBuilder::new(self.config.clone())
    }

    pub fn patch(&self) -> crate::vc::vc::v1::room::PatchRoomRequestBuilder {
        crate::vc::vc::v1::room::PatchRoomRequestBuilder::new(self.config.clone())
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

    pub fn apply(&self) -> crate::vc::vc::v1::reserve::ApplyReserveRequestBuilder {
        crate::vc::vc::v1::reserve::ApplyReserveRequestBuilder::new(self.config.clone())
    }

    pub fn delete_reserve(&self) -> crate::vc::vc::v1::reserve::DeleteReserveRequestBuilder {
        crate::vc::vc::v1::reserve::DeleteReserveRequestBuilder::new(self.config.clone())
    }

    pub fn get_reserve(&self) -> crate::vc::vc::v1::reserve::GetReserveRequestBuilder {
        crate::vc::vc::v1::reserve::GetReserveRequestBuilder::new(self.config.clone())
    }

    pub fn update_reserve(&self) -> crate::vc::vc::v1::reserve::UpdateReserveRequestBuilder {
        crate::vc::vc::v1::reserve::UpdateReserveRequestBuilder::new(self.config.clone())
    }
}
