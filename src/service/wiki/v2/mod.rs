use crate::core::config::Config;

pub use space::SpaceService;
pub use space_member::SpaceMemberService;

pub mod space;
pub mod space_member;

pub struct V2 {
    /// 知识空间
    pub space: SpaceService,
    /// 空间成员
    pub space_member: SpaceMemberService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            space: SpaceService::new(config.clone()),
            space_member: SpaceMemberService::new(config),
        }
    }
}