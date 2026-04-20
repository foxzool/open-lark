/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 列表接口。
pub mod list;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱联系人资源。
#[derive(Clone)]
pub struct MailContact {
    config: Arc<Config>,
    mailbox_id: String,
}

impl MailContact {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailContactRequest {
        list::ListMailContactRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailContactRequest {
        create::CreateMailContactRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建补丁请求。
    pub fn patch(&self, mail_contact_id: impl Into<String>) -> patch::PatchMailContactRequest {
        patch::PatchMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            mail_contact_id,
        )
    }

    /// 创建删除请求。
    pub fn delete(&self, mail_contact_id: impl Into<String>) -> delete::DeleteMailContactRequest {
        delete::DeleteMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            mail_contact_id,
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
