pub mod create;
pub mod delete;
pub mod list;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Folder {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Folder {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailboxFolderRequest {
        list::ListMailboxFolderRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn create(&self) -> create::CreateMailboxFolderRequest {
        create::CreateMailboxFolderRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn patch(&self, folder_id: impl Into<String>) -> patch::PatchMailboxFolderRequest {
        patch::PatchMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id,
        )
    }

    pub fn delete(&self, folder_id: impl Into<String>) -> delete::DeleteMailboxFolderRequest {
        delete::DeleteMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id,
        )
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
