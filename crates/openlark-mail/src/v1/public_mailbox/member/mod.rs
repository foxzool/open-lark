//! 公共邮箱成员模块

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// Member：公共邮箱成员资源
#[derive(Clone)]
pub struct Member {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Member {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建成员
    pub fn create(&self) -> create::CreatePublicMailboxMemberRequest {
        create::CreatePublicMailboxMemberRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取成员详情
    pub fn get(&self, member_id: impl Into<String>) -> get::GetPublicMailboxMemberRequest {
        get::GetPublicMailboxMemberRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            member_id.into(),
        )
    }

    /// 获取成员列表
    pub fn list(&self) -> list::PublicMailboxMemberListRequest {
        list::PublicMailboxMemberListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除成员
    pub fn delete(&self, member_id: impl Into<String>) -> delete::DeletePublicMailboxMemberRequest {
        delete::DeletePublicMailboxMemberRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            member_id.into(),
        )
    }
}

// 重新导出
pub use create::CreatePublicMailboxMemberRequest;
pub use delete::DeletePublicMailboxMemberRequest;
pub use get::GetPublicMailboxMemberRequest;
pub use list::PublicMailboxMemberListRequest;
pub use models::{
    CreatePublicMailboxMemberBody, CreatePublicMailboxMemberResponse, DeletePublicMailboxMemberResponse,
    GetPublicMailboxMemberResponse, PublicMailboxMemberItem, PublicMailboxMemberListResponse,
};
