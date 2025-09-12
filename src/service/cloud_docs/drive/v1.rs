use std::sync::Arc;

use crate::core::config::Config;

pub mod event;
pub mod file;
pub mod file_version;
pub mod files;
pub mod folder;
pub mod like;
pub mod media;
pub mod permissions;

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
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            event: event::EventService::new(Arc::clone(&config)),
            file: file::FileService::new(Arc::clone(&config)),
            file_version: file_version::FileVersionService::new(Arc::clone(&config)),
            files: files::FilesService::new(Arc::clone(&config)),
            folder: folder::FolderService::new(Arc::clone(&config)),
            like: like::LikeService::new(Arc::clone(&config)),
            media: media::MediaService::new(Arc::clone(&config)),
            permissions: permissions::PermissionsService::new(config),
        }
    }
}
