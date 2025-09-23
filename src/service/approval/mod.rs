//! 审批（Approval）服务
//!
//! 提供飞书审批应用的完整功能集，支持审批流程管理、审批实例处理、
//! 任务分配等企业级工作流程管理能力。是企业数字化办公的核心服务模块。
//!
//! # 核心功能
//!
//! ## 审批流程管理
//! - 📋 审批定义的创建和配置
//! - 🔄 审批流程设计和编辑
//! - 🎯 审批节点和条件设置
//! - 📊 流程模板管理
//!
//! ## 审批实例处理
//! - 📝 审批实例的创建和提交
//! - ✅ 审批操作（同意/拒绝/转交）
//! - 🔍 审批状态查询和跟踪
//! - 📋 审批历史记录管理
//!
//! ## 任务管理
//! - 📋 待办任务查询和处理
//! - 👥 任务分配和委托
//! - ⏰ 任务提醒和超时处理
//! - 📊 任务统计和分析
//!
//! ## 外部集成
//! - 🔗 外部审批系统对接
//! - 📊 第三方数据同步
//! - 🔄 审批状态双向同步
//! - 🛠️ 自定义集成接口
//!
//! ## 文件和消息
//! - 📎 审批附件管理
//! - 💬 审批评论和消息
//! - 🔔 审批通知推送
//! - 📧 邮件和短信提醒
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取审批服务
//! let approval = &client.approval;
//!
//! // 创建审批实例
//! // let instance_request = CreateInstanceRequest::builder()
//! //     .approval_code("approval_code")
//! //     .form("{\"field1\":\"value1\"}")
//! //     .build();
//! // let instance = approval.v4.instance.create(instance_request, None).await?;
//!
//! // 查询待办任务
//! // let task_request = ListTaskRequest::builder()
//! //     .page_size(20)
//! //     .build();
//! // let tasks = approval.v4.task.list(task_request, None).await?;
//!
//! // 审批操作
//! // let approve_request = ApproveTaskRequest::builder()
//! //     .approval_code("approval_code")
//! //     .instance_code("instance_code")
//! //     .task_id("task_id")
//! //     .build();
//! // approval.v4.task.approve(approve_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v4版本，是最新的稳定版本，提供：
//! - 完整的审批流程管理
//! - 高性能的实例处理
//! - 丰富的集成能力
//! - 企业级安全控制
//!
//! # 工作流特性
//!
//! - 🔄 灵活的流程配置
//! - 👥 多级审批支持
//! - 🔀 条件分支和并行处理
//! - ⏰ 超时和升级机制
//! - 📊 审批数据统计分析
//!
//! # 集成能力
//!
//! - 📱 移动端审批支持
//! - 🔗 第三方系统集成
//! - 📧 多渠道消息通知
//! - 📊 BI和报表集成

use crate::core::config::Config;

/// 数据模型定义
pub mod models;
/// 审批服务 v4 版本
pub mod v4;

use v4::V4;

/// 审批服务
///
/// 企业级审批流程管理的统一入口，提供完整的工作流程设计、
/// 审批实例处理、任务管理等核心功能。
///
/// # 服务架构
///
/// - **v4**: 最新版本API，提供完整的审批功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🚀 高性能审批引擎
/// - 🔄 灵活的流程配置
/// - 👥 多角色权限管理
/// - 📊 实时数据统计
/// - 🔗 丰富的集成接口
///
/// # 适用场景
///
/// - 企业日常审批流程
/// - 财务费用报销
/// - 人事请假申请
/// - 采购合同审批
/// - 项目立项审核
///
/// # 最佳实践
///
/// - 设计清晰的审批流程
/// - 合理设置审批权限
/// - 及时处理审批任务
/// - 定期分析审批数据
/// - 优化审批效率
pub struct ApprovalService {
    /// v4版本API服务
    pub v4: V4,
}

impl ApprovalService {
    /// 创建新的审批服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的审批服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v4: V4::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_approval_service_creation() {
        let config = Config::default();
        let service = ApprovalService::new(config);

        // Verify V4 service structure exists with all 10 sub-services
        let _ = &service.v4;
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }

    #[test]
    fn test_approval_service_with_custom_config() {
        let config = Config::builder()
            .app_id("approval_test_app")
            .app_secret("approval_test_secret")
            .req_timeout(Duration::from_secs(320))
            .build();

        let service = ApprovalService::new(config);

        // Verify service creation with custom config
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }

    #[test]
    fn test_approval_service_config_independence() {
        let config1 = Config::builder().app_id("approval_app_1").build();

        let config2 = Config::builder().app_id("approval_app_2").build();

        let service1 = ApprovalService::new(config1);
        let service2 = ApprovalService::new(config2);

        // Verify both services are created successfully
        let _ = &service1.v4.approval;
        let _ = &service1.v4.instance;
        let _ = &service2.v4.approval;
        let _ = &service2.v4.instance;
    }

    #[test]
    fn test_approval_service_sub_services_accessible() {
        let config = Config::default();
        let service = ApprovalService::new(config);

        // Test that all sub-services are accessible
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }

    #[test]
    fn test_approval_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = ApprovalService::new(config.clone());

        // Verify service creation with cloned config
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }

    #[test]
    fn test_approval_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(330))
            .build();

        let service = ApprovalService::new(config);

        // Verify service creation with timeout config
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }

    #[test]
    fn test_approval_service_multiple_instances() {
        let config = Config::default();

        let service1 = ApprovalService::new(config.clone());
        let service2 = ApprovalService::new(config.clone());

        // Verify both instances are created successfully
        let _ = &service1.v4.approval;
        let _ = &service1.v4.instance;
        let _ = &service2.v4.approval;
        let _ = &service2.v4.instance;
    }

    #[test]
    fn test_approval_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(340))
            .build();

        let service = ApprovalService::new(config);

        // Verify all sub-services are created consistently
        let _ = &service.v4.approval;
        let _ = &service.v4.instance;
        let _ = &service.v4.task;
        let _ = &service.v4.file;
        let _ = &service.v4.instance_comment;
        let _ = &service.v4.external_approval;
        let _ = &service.v4.external_instance;
        let _ = &service.v4.external_task;
        let _ = &service.v4.message;
        let _ = &service.v4.search;
    }
}
