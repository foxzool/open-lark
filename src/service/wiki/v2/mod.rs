use crate::core::config::Config;

pub use space::SpaceService;
pub use space_member::SpaceMemberService;
pub use space_setting::SpaceSettingService;

pub mod space;
pub mod space_member;
pub mod space_setting;

pub struct V2 {
    /// 知识空间
    pub space: SpaceService,
    /// 空间成员
    pub space_member: SpaceMemberService,
    /// 空间设置
    pub space_setting: SpaceSettingService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            space: SpaceService::new(config.clone()),
            space_member: SpaceMemberService::new(config.clone()),
            space_setting: SpaceSettingService::new(config),
        }
    }
}