pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod upload;

use openlark_core::config::Config;
use std::sync::Arc;

/// Attachment：附件资源
#[derive(Clone)]
pub struct Attachment {
    config: Arc<Config>,
    task_guid: String,
}

impl Attachment {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            task_guid: String::new(),
        }
    }

    pub fn with_task(mut self, task_guid: impl Into<String>) -> Self {
        self.task_guid = task_guid.into();
        self
    }

    pub fn upload(&self, file_path: String) -> upload::UploadAttachmentRequest {
        upload::UploadAttachmentRequest::new(self.config.clone(), self.task_guid.clone(), file_path)
    }

    pub fn delete(&self, attachment_guid: impl Into<String>) -> delete::DeleteAttachmentRequest {
        delete::DeleteAttachmentRequest::new(
            self.config.clone(),
            self.task_guid.clone(),
            attachment_guid.into(),
        )
    }

    /// 获取附件（不需要 task_guid）
    pub fn get(&self, attachment_guid: impl Into<String>) -> get::GetAttachmentRequest {
        get::GetAttachmentRequest::new(self.config.clone(), attachment_guid.into())
    }

    /// 列取附件（不需要 task_guid）
    pub fn list(&self) -> list::ListAttachmentsRequest {
        list::ListAttachmentsRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use delete::DeleteAttachmentRequest;
pub use get::GetAttachmentRequest;
pub use list::ListAttachmentsRequest;
pub use upload::UploadAttachmentRequest;

// 重新导出响应类型
pub use get::{AttachmentInfo, GetAttachmentResponse};
pub use list::{AttachmentListItem, ListAttachmentsResponse};
pub use models::{DeleteAttachmentResponse, UploadAttachmentResponse};
