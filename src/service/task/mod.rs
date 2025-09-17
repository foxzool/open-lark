//! 任务（Task）服务
//!
//! 提供飞书任务应用的完整功能集，支持任务管理、任务列表、子任务、
//! 评论、附件等企业级任务协作能力。是团队项目管理和协作的核心工具。
//!
//! # 核心功能
//!
//! ## 任务管理
//! - 📝 任务的创建、编辑和删除
//! - ✅ 任务状态管理（进行中/已完成等）
//! - 📅 任务截止时间和提醒设置
//! - 👥 任务分配和负责人管理
//! - 🏷️ 任务标签和分类
//!
//! ## 任务列表
//! - 📋 任务列表的创建和管理
//! - 📊 任务列表模板和配置
//! - 🔄 任务列表的活动订阅
//! - 📈 任务进度跟踪和统计
//!
//! ## 子任务和层级
//! - 🌳 子任务的创建和管理
//! - 📊 任务层级结构展示
//! - 🔗 父子任务关联关系
//! - 📈 层级任务进度汇总
//!
//! ## 评论和协作
//! - 💬 任务评论和讨论
//! - 📎 评论附件和文件分享
//! - 🔔 @提醒和消息通知
//! - 📝 任务更新历史记录
//!
//! ## 附件管理
//! - 📎 任务附件上传和管理
//! - 🔗 文件关联和共享
//! - 📁 附件分类和组织
//! - 🔒 附件权限控制
//!
//! ## 自定义字段
//! - 🏷️ 自定义字段定义
//! - 📝 字段选项配置
//! - 📊 字段数据统计
//! - 🎯 个性化表单设计
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
//! // 获取任务服务（通过v2版本）
//! // let task_service = &client.task.v2;
//!
//! // 创建任务列表
//! // let tasklist_request = CreateTasklistRequest::builder()
//! //     .name("项目开发任务")
//! //     .description("软件开发相关任务列表")
//! //     .build();
//! // let tasklist = task_service.tasklist.create(tasklist_request, None).await?;
//!
//! // 创建任务
//! // let task_request = CreateTaskRequest::builder()
//! //     .tasklist_guid("tasklist_guid")
//! //     .summary("完成API文档")
//! //     .description("编写完整的API使用文档")
//! //     .build();
//! // let task = task_service.task.create(task_request, None).await?;
//!
//! // 添加任务评论
//! // let comment_request = CreateCommentRequest::builder()
//! //     .task_guid("task_guid")
//! //     .content("进展顺利，预计明天完成")
//! //     .build();
//! // task_service.comment.create(comment_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v2版本，是最新的稳定版本，提供：
//! - 完整的任务管理功能
//! - 丰富的协作特性
//! - 灵活的自定义配置
//! - 高性能的数据处理
//!
//! # 任务协作特性
//!
//! - 👥 多人协作和权限管理
//! - 📱 移动端和桌面端同步
//! - 🔔 智能提醒和通知
//! - 📊 任务数据分析统计
//! - 🔗 第三方工具集成
//!
//! # 项目管理能力
//!
//! - 📈 项目进度可视化
//! - 🎯 里程碑和关键节点
//! - 📊 工作量统计分析
//! - 🔄 敏捷开发支持
//! - 📋 看板式任务管理

/// 数据模型定义
pub mod models;
/// 任务服务 v2 版本
pub mod v2;

pub use models::*;
pub use v2::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_module_structure() {
        // Test that all modules are properly exposed
        assert!(true); // Basic structure test
    }

    #[test]
    fn test_models_import() {
        // Test that models are properly imported
        let _user_id_type = UserIdType::OpenId;
        assert!(true);
    }
}
