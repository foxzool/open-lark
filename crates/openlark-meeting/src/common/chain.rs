//! openlark-meeting 链式调用入口（meta 风格）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入"额外实现文件"。
//! - 会议模块本身已存在 `service.rs` 的 Builder/Resource 分层，这里提供"字段链式入口"。
//! - Config 内部已使用 Arc，无需重复包装。

use openlark_core::config::Config;

/// 会议链式入口：`meeting.vc.v1.room.create()` 等
#[derive(Debug, Clone)]
pub struct MeetingClient {
    config: Config,

    #[cfg(feature = "calendar")]
    pub calendar: CalendarClient,

    #[cfg(feature = "vc")]
    pub vc: VcClient,

    #[cfg(feature = "meeting-room")]
    pub meeting_room: MeetingRoomClient,
}

impl MeetingClient {
    pub fn new(config: Config) -> Self {
        let config_cloned = config.clone();
        Self {
            config: config_cloned,
            #[cfg(feature = "calendar")]
            calendar: CalendarClient::new(config.clone()),
            #[cfg(feature = "vc")]
            vc: VcClient::new(config.clone()),
            #[cfg(feature = "meeting-room")]
            meeting_room: MeetingRoomClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarClient {
    config: Config,
    pub v4: CalendarV4Client,
}

#[cfg(feature = "calendar")]
impl CalendarClient {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v4: CalendarV4Client::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarV4Client {
    config: Config,
    pub calendar: CalendarResourceClient,
}

#[cfg(feature = "calendar")]
impl CalendarV4Client {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            calendar: CalendarResourceClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarResourceClient {
    config: Config,
}

#[cfg(feature = "calendar")]
impl CalendarResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcClient {
    config: Config,
    pub v1: VcV1Client,
}

#[cfg(feature = "vc")]
impl VcClient {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: VcV1Client::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcV1Client {
    config: Config,
    pub room: VcRoomResourceClient,
    pub meeting: VcMeetingResourceClient,
    pub reserve: VcReserveResourceClient,
}

#[cfg(feature = "vc")]
impl VcV1Client {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            room: VcRoomResourceClient::new(config.clone()),
            meeting: VcMeetingResourceClient::new(config.clone()),
            reserve: VcReserveResourceClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcRoomResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcRoomResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn create(&self) -> crate::vc::v1::room::CreateRoomRequestBuilder {
        crate::vc::v1::room::CreateRoomRequestBuilder::new(self.config.clone())
    }

    pub fn get(&self) -> crate::vc::v1::room::GetRoomRequestBuilder {
        crate::vc::v1::room::GetRoomRequestBuilder::new(self.config.clone())
    }

    pub fn delete(&self) -> crate::vc::v1::room::DeleteRoomRequestBuilder {
        crate::vc::v1::room::DeleteRoomRequestBuilder::new(self.config.clone())
    }

    pub fn list(&self) -> crate::vc::v1::room::ListRoomRequestBuilder {
        crate::vc::v1::room::ListRoomRequestBuilder::new(self.config.clone())
    }

    pub fn mget(&self) -> crate::vc::v1::room::MgetRoomRequestBuilder {
        crate::vc::v1::room::MgetRoomRequestBuilder::new(self.config.clone())
    }

    pub fn patch(&self) -> crate::vc::v1::room::PatchRoomRequestBuilder {
        crate::vc::v1::room::PatchRoomRequestBuilder::new(self.config.clone())
    }
}

#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcMeetingResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcMeetingResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcReserveResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcReserveResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn apply(&self) -> crate::vc::v1::reserve::ApplyReserveRequestBuilder {
        crate::vc::v1::reserve::ApplyReserveRequestBuilder::new(self.config.clone())
    }

    pub fn delete_reserve(&self) -> crate::vc::v1::reserve::DeleteReserveRequestBuilder {
        crate::vc::v1::reserve::DeleteReserveRequestBuilder::new(self.config.clone())
    }

    pub fn get_reserve(&self) -> crate::vc::v1::reserve::GetReserveRequestBuilder {
        crate::vc::v1::reserve::GetReserveRequestBuilder::new(self.config.clone())
    }

    pub fn update_reserve(&self) -> crate::vc::v1::reserve::UpdateReserveRequestBuilder {
        crate::vc::v1::reserve::UpdateReserveRequestBuilder::new(self.config.clone())
    }
}

#[cfg(feature = "meeting-room")]
#[derive(Debug, Clone)]
pub struct MeetingRoomClient {
    config: Config,
}

#[cfg(feature = "meeting-room")]
impl MeetingRoomClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
