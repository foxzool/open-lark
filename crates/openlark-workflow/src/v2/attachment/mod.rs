pub mod delete;
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
        delete::DeleteAttachmentRequest::new(self.config.clone(), self.task_guid.clone(), attachment_guid.into())
    }
}

// 重新导出请求类型
pub use delete::DeleteAttachmentRequest;
pub use upload::UploadAttachmentRequest;

// 重新导出响应类型
pub use models::{DeleteAttachmentResponse, UploadAttachmentResponse};
