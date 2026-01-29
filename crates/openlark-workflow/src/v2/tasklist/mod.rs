pub mod activity_subscription;
pub mod add_members;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod remove_members;
pub mod tasks;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Tasklist：任务清单资源
#[derive(Clone)]
pub struct Tasklist {
    config: Arc<Config>,
}

impl Tasklist {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateTasklistRequest {
        create::CreateTasklistRequest::new(self.config.clone())
    }

    pub fn get(&self, tasklist_guid: impl Into<String>) -> get::GetTasklistRequest {
        get::GetTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn update(&self, tasklist_guid: impl Into<String>) -> update::UpdateTasklistRequest {
        update::UpdateTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn delete(&self, tasklist_guid: impl Into<String>) -> delete::DeleteTasklistRequest {
        delete::DeleteTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn list(&self) -> list::ListTasklistsRequest {
        list::ListTasklistsRequest::new(self.config.clone())
    }

    /// 获取动态订阅资源
    pub fn activity_subscription(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> activity_subscription::ActivitySubscriptionResource {
        activity_subscription::ActivitySubscriptionResource::new(self.config.clone(), tasklist_guid.into())
    }

    /// 获取清单任务列表
    pub fn tasks(&self, tasklist_guid: impl Into<String>) -> tasks::GetTasklistTasksRequest {
        tasks::GetTasklistTasksRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 添加清单成员
    pub fn add_members(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> add_members::AddTasklistMembersRequest {
        add_members::AddTasklistMembersRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 移除清单成员
    pub fn remove_members(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> remove_members::RemoveTasklistMembersRequest {
        remove_members::RemoveTasklistMembersRequest::new(self.config.clone(), tasklist_guid.into())
    }
}

// 重新导出请求类型
pub use add_members::AddTasklistMembersRequest;
pub use create::CreateTasklistRequest;
pub use delete::DeleteTasklistRequest;
pub use get::GetTasklistRequest;
pub use list::ListTasklistsRequest;
pub use patch::UpdateTasklistRequest;
pub use remove_members::RemoveTasklistMembersRequest;
pub use tasks::GetTasklistTasksRequest;

// 重新导出响应类型
pub use add_members::{AddTasklistMembersBody, AddTasklistMembersResponse, TasklistMember};
pub use models::{
    CreateTasklistBody, CreateTasklistResponse, DeleteTasklistResponse, GetTasklistResponse,
    ListTasklistsResponse, TasklistIcon, TasklistItem, UpdateTasklistBody, UpdateTasklistResponse,
};
pub use remove_members::{RemoveTasklistMembersBody, RemoveTasklistMembersResponse};
