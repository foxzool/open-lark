use crate::core::config::Config;

pub mod explorer;

pub struct V2 {
    pub explorer: explorer::ExplorerService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            explorer: explorer::ExplorerService::new(config),
        }
    }
}
