//! 动态订阅模块

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// ActivitySubscription：动态订阅资源
#[derive(Clone)]
pub struct ActivitySubscriptionResource {
    config: Arc<Config>,
    tasklist_guid: String,
}

impl ActivitySubscriptionResource {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
        }
    }

    /// 创建动态订阅
    pub fn create(&self) -> create::CreateActivitySubscriptionRequest {
        create::CreateActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
        )
    }

    /// 获取动态订阅
    pub fn get(
        &self,
        subscription_guid: impl Into<String>,
    ) -> get::GetActivitySubscriptionRequest {
        get::GetActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 更新动态订阅
    pub fn patch(
        &self,
        subscription_guid: impl Into<String>,
    ) -> patch::UpdateActivitySubscriptionRequest {
        patch::UpdateActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 删除动态订阅
    pub fn delete(
        &self,
        subscription_guid: impl Into<String>,
    ) -> delete::DeleteActivitySubscriptionRequest {
        delete::DeleteActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 列取动态订阅
    pub fn list(&self) -> list::ListActivitySubscriptionsRequest {
        list::ListActivitySubscriptionsRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateActivitySubscriptionRequest;
pub use delete::DeleteActivitySubscriptionRequest;
pub use get::GetActivitySubscriptionRequest;
pub use list::ListActivitySubscriptionsRequest;
pub use patch::UpdateActivitySubscriptionRequest;

// 重新导出响应类型
pub use models::{
    ActivitySubscription, ActivitySubscriptionTargetType, ActivitySubscriptionType,
    CreateActivitySubscriptionBody, CreateActivitySubscriptionResponse,
    DeleteActivitySubscriptionResponse, GetActivitySubscriptionResponse,
    ListActivitySubscriptionsResponse, UpdateActivitySubscriptionBody,
    UpdateActivitySubscriptionResponse,
};
