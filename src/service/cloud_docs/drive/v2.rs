use crate::{
    core::config::Config,
    service::drive::{v1::permissions::PermissionsService, v2::explorer::ExplorerService},
};

pub mod explorer;

pub struct V2 {
    pub explorer: ExplorerService,
    pub permission: PermissionsService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            explorer: ExplorerService::new(config.clone()),
            permission: PermissionsService::new(config),
        }
    }
}
