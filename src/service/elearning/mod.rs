//! # 飞书在线学习服务
//!
//! 飞书在线学习 (eLearning) 服务提供课程学习进度管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **课程学习进度管理**：创建、查询、更新和删除学习进度记录
//! - **学习数据统计**：获取学习状态和进度统计
//! - **事件支持**：支持学习进度变更事件处理
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`course_registration`] - 课程学习进度管理模块
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use open_lark::prelude::*;
//! use open_lark::service::elearning::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 查询课程学习进度
//!     let registrations = client.elearning.course_registration.list(
//!         course_registration::CourseRegistrationListRequest::default(), None
//!     ).await?;
//!     
//!     // 创建学习进度记录
//!     let create_req = course_registration::CourseRegistrationCreateRequest {
//!         course_id: "course_123".to_string(),
//!         user_id: "user_456".to_string(),
//!         registration_type: Some("manual".to_string()),
//!     };
//!     let result = client.elearning.course_registration.create(create_req, None).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod course_registration;
pub mod models;

use crate::{
    core::config::Config, service::elearning::course_registration::CourseRegistrationService,
};

/// 飞书在线学习服务
///
/// 提供完整的在线学习功能，包括课程学习进度管理
pub struct ELearningService {
    /// 课程学习进度管理服务
    pub course_registration: CourseRegistrationService,
}

impl ELearningService {
    /// 创建飞书在线学习服务实例
    pub fn new(config: Config) -> Self {
        Self {
            course_registration: CourseRegistrationService::new(config),
        }
    }
}
