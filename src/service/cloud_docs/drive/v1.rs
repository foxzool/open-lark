use crate::core::config::Config;

pub mod file;
pub mod file_version;
pub mod files;
pub mod folder;
pub mod media;
pub mod permissions;

pub struct V1 {
    pub file: file::FileService,
    pub file_version: file_version::FileVersionService,
    pub files: files::FilesService,
    pub folder: folder::FolderService,
    pub media: media::MediaService,
    pub permissions: permissions::PermissionsService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            file: file::FileService::new(config.clone()),
            file_version: file_version::FileVersionService::new(config.clone()),
            files: files::FilesService::new(config.clone()),
            folder: folder::FolderService::new(config.clone()),
            media: media::MediaService::new(config.clone()),
            permissions: permissions::PermissionsService::new(config),
        }
    }
}
