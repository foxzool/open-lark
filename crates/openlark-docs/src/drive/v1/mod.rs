pub mod export_task;
pub mod file;
pub mod import_task;
pub mod media;
pub mod meta;
pub mod permission;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct DriveV1 {
    service: Arc<DocsService>,
}

impl DriveV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn export_task(&self) -> export_task::ExportTask {
        export_task::ExportTask::new(self.service.clone())
    }

    pub fn file(&self) -> file::File {
        file::File::new(self.service.clone())
    }

    pub fn file_comment(&self) -> file::comment::FileComment {
        file::comment::FileComment::new(self.service.clone())
    }

    pub fn file_comment_reply(&self) -> file::comment::reply::FileCommentReply {
        file::comment::reply::FileCommentReply::new(self.service.clone())
    }

    pub fn file_statistics(&self) -> file::statistics::FileStatistics {
        file::statistics::FileStatistics::new(self.service.clone())
    }

    pub fn file_subscription(&self) -> file::subscription::FileSubscription {
        file::subscription::FileSubscription::new(self.service.clone())
    }

    pub fn file_version(&self) -> file::version::FileVersion {
        file::version::FileVersion::new(self.service.clone())
    }

    pub fn file_view_record(&self) -> file::view_record::FileViewRecord {
        file::view_record::FileViewRecord::new(self.service.clone())
    }

    pub fn import_task(&self) -> import_task::ImportTask {
        import_task::ImportTask::new(self.service.clone())
    }

    pub fn media(&self) -> media::Media {
        media::Media::new(self.service.clone())
    }

    pub fn meta(&self) -> meta::Meta {
        meta::Meta::new(self.service.clone())
    }

    pub fn permission_member(&self) -> permission::member::PermissionMember {
        permission::member::PermissionMember::new(self.service.clone())
    }

    pub fn permission_public(&self) -> permission::public::PermissionPublic {
        permission::public::PermissionPublic::new(self.service.clone())
    }

    pub fn permission_public_password(&self) -> permission::public::password::PermissionPublicPassword {
        permission::public::password::PermissionPublicPassword::new(self.service.clone())
    }
}
