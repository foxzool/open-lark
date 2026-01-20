pub mod create;
pub mod get;
pub mod update;
pub mod delete;
pub mod list;
pub mod models;

use std::sync::Arc;
use openlark_core::config::Config;

/// MailGroup：邮件组资源（v1）
#[derive(Clone)]
pub struct MailGroup {
    config: Arc<Config>,
}

impl MailGroup {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateMailGroupRequest {
        create::CreateMailGroupRequest::new(self.config.clone())
    }

    pub fn get(&self, mail_group_id: impl Into<String>) -> get::GetMailGroupRequest {
        get::GetMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn update(&self, mail_group_id: impl Into<String>) -> update::UpdateMailGroupRequest {
        update::UpdateMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn delete(&self, mail_group_id: impl Into<String>) -> delete::DeleteMailGroupRequest {
        delete::DeleteMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn list(&self) -> list::MailGroupListRequest {
        list::MailGroupListRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use create::CreateMailGroupRequest;
pub use get::GetMailGroupRequest;
pub use update::UpdateMailGroupRequest;
pub use delete::DeleteMailGroupRequest;
pub use list::MailGroupListRequest;

// 重新导出响应类型
pub use models::{
    CreateMailGroupBody, CreateMailGroupResponse,
    GetMailGroupResponse, UpdateMailGroupBody,
    UpdateMailGroupResponse, DeleteMailGroupResponse,
    MailGroupListResponse, MailGroupItem,
};
