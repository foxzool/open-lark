//! # 工作台服务
//!
//! 工作台 (Workplace) 服务提供工作台访问数据和应用推荐功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **工作台访问数据**：获取工作台访问数据、定制工作台访问数据、定制工作台小组件访问数据
//! - **我的常用应用**：获取用户自定义常用应用、管理员推荐应用、推荐规则列表
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`workplace_access_data`] - 工作台访问数据模块
//! - [`app_recommend`] - 应用推荐模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::workplace::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取工作台访问数据
//!     let access_data = client.workplace.workplace_access_data.search(
//!         workplace_access_data::AccessDataSearchRequest::default(), None
//!     ).await?;
//!     
//!     // 获取用户常用应用
//!     let favourite_apps = client.workplace.app_recommend.get_favourite_apps(
//!         app_recommend::FavouriteAppsRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod app_recommend;
pub mod models;
pub mod workplace_access_data;

use crate::{
    core::config::Config,
    service::workplace::{
        app_recommend::AppRecommendService, workplace_access_data::WorkplaceAccessDataService,
    },
};

/// 工作台服务
///
/// 提供完整的工作台功能，包括访问数据统计和应用推荐管理
pub struct WorkplaceService {
    /// 工作台访问数据服务
    pub workplace_access_data: WorkplaceAccessDataService,
    /// 应用推荐服务
    pub app_recommend: AppRecommendService,
}

impl WorkplaceService {
    /// 创建工作台服务实例
    pub fn new(config: Config) -> Self {
        Self {
            workplace_access_data: WorkplaceAccessDataService::new(config.clone()),
            app_recommend: AppRecommendService::new(config),
        }
    }
}
