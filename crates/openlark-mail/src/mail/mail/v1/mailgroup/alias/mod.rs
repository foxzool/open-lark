pub mod create;
pub mod delete;
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 邮件组别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailgroup_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailGroupAliasRequest {
        list::ListMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn create(&self) -> create::CreateMailGroupAliasRequest {
        create::CreateMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeleteMailGroupAliasRequest {
        delete::DeleteMailGroupAliasRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
            alias_id,
        )
    }
}
