use crate::core::config::Config;

pub mod event;
pub mod file;
pub mod file_version;
pub mod files;
pub mod folder;
pub mod like;
pub mod media;
pub mod permissions;

// Drive v1 事件模块
pub mod p2_drive_file_created_v1;
pub mod p2_drive_file_deleted_v1;
pub mod p2_drive_file_updated_v1;

pub struct V1 {
    pub event: event::EventService,
    pub file: file::FileService,
    pub file_version: file_version::FileVersionService,
    pub files: files::FilesService,
    pub folder: folder::FolderService,
    pub like: like::LikeService,
    pub media: media::MediaService,
    pub permissions: permissions::PermissionsService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            event: event::EventService::new(config.clone()),
            file: file::FileService::new(config.clone()),
            file_version: file_version::FileVersionService::new(config.clone()),
            files: files::FilesService::new(config.clone()),
            folder: folder::FolderService::new(config.clone()),
            like: like::LikeService::new(config.clone()),
            media: media::MediaService::new(config.clone()),
            permissions: permissions::PermissionsService::new(config),
        }
    }
}
