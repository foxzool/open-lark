pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 邮件组权限成员资源
#[derive(Clone)]
pub struct PermissionMember {
    config: Arc<Config>,
    mailgroup_id: String,
}

impl PermissionMember {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailGroupPermissionMemberRequest {
        list::ListMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn get(&self, permission_member_id: impl Into<String>) -> get::GetMailGroupPermissionMemberRequest {
        get::GetMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone(), permission_member_id)
    }

    pub fn create(&self) -> create::CreateMailGroupPermissionMemberRequest {
        create::CreateMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn delete(&self, permission_member_id: impl Into<String>) -> delete::DeleteMailGroupPermissionMemberRequest {
        delete::DeleteMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone(), permission_member_id)
    }

    pub fn batch_create(&self) -> batch_create::BatchCreateMailGroupPermissionMemberRequest {
        batch_create::BatchCreateMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn batch_delete(&self) -> batch_delete::BatchDeleteMailGroupPermissionMemberRequest {
        batch_delete::BatchDeleteMailGroupPermissionMemberRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }
}
