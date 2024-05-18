use crate::core::config::Config;

pub mod files;

pub struct V1 {
    pub files: files::FilesService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            files: files::FilesService::new(config),
        }
    }
}
