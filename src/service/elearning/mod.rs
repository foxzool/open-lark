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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_elearning_service_creation() {
        let config = Config::default();
        let service = ELearningService::new(config.clone());

        assert_eq!(service.course_registration.config.app_id, config.app_id);
        assert_eq!(
            service.course_registration.config.app_secret,
            config.app_secret
        );
    }

    #[test]
    fn test_elearning_service_with_custom_config() {
        let config = Config::builder()
            .app_id("elearning_test_app")
            .app_secret("elearning_test_secret")
            .req_timeout(Duration::from_secs(90))
            .build();

        let service = ELearningService::new(config.clone());

        assert_eq!(
            service.course_registration.config.app_id,
            "elearning_test_app"
        );
        assert_eq!(
            service.course_registration.config.app_secret,
            "elearning_test_secret"
        );
        assert_eq!(
            service.course_registration.config.req_timeout,
            Some(Duration::from_secs(90))
        );
    }

    #[test]
    fn test_elearning_service_config_independence() {
        let config1 = Config::builder().app_id("elearning_app_1").build();

        let config2 = Config::builder().app_id("elearning_app_2").build();

        let service1 = ELearningService::new(config1);
        let service2 = ELearningService::new(config2);

        assert_eq!(
            service1.course_registration.config.app_id,
            "elearning_app_1"
        );
        assert_eq!(
            service2.course_registration.config.app_id,
            "elearning_app_2"
        );
        assert_ne!(
            service1.course_registration.config.app_id,
            service2.course_registration.config.app_id
        );
    }

    #[test]
    fn test_elearning_service_sub_services_accessible() {
        let config = Config::default();
        let service = ELearningService::new(config.clone());

        assert_eq!(service.course_registration.config.app_id, config.app_id);
    }

    #[test]
    fn test_elearning_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = ELearningService::new(config.clone());

        assert_eq!(service.course_registration.config.app_id, "clone_test_app");
        assert_eq!(
            service.course_registration.config.app_secret,
            "clone_test_secret"
        );
    }

    #[test]
    fn test_elearning_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = ELearningService::new(config);

        assert_eq!(
            service.course_registration.config.req_timeout,
            Some(Duration::from_secs(200))
        );
    }

    #[test]
    fn test_elearning_service_multiple_instances() {
        let config = Config::default();

        let service1 = ELearningService::new(config.clone());
        let service2 = ELearningService::new(config.clone());

        assert_eq!(
            service1.course_registration.config.app_id,
            service2.course_registration.config.app_id
        );
        assert_eq!(
            service1.course_registration.config.app_secret,
            service2.course_registration.config.app_secret
        );
    }

    #[test]
    fn test_elearning_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(100))
            .build();

        let service = ELearningService::new(config);

        assert_eq!(
            service.course_registration.config.app_id,
            "consistency_test"
        );
        assert_eq!(
            service.course_registration.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.course_registration.config.req_timeout,
            Some(Duration::from_secs(100))
        );
    }
}
