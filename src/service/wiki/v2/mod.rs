use crate::core::config::Config;

pub use space::SpaceService;

pub mod space;

pub struct V2 {
    /// 知识空间
    pub space: SpaceService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            space: SpaceService::new(config),
        }
    }
}