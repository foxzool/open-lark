use crate::core::config::Config;

pub mod files;
pub mod folder;
pub mod permissions;

pub struct V1 {
    pub files: files::FilesService,
    pub folder: folder::FolderService,
    pub permissions: permissions::PermissionsService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            files: files::FilesService::new(config.clone()),
            folder: folder::FolderService::new(config.clone()),
            permissions: permissions::PermissionsService::new(config),
        }
    }
}
