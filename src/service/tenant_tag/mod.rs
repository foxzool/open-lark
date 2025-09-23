//! 企业自定义群标签（Tenant Tag）服务
//!
//! 提供飞书企业自定义群标签的完整功能集，支持标签管理、标签绑定、
//! 群组分类等企业级群组标签化管理能力。是企业群组精细化管理的重要工具。
//!
//! # 核心功能
//!
//! ## 标签管理
//! - 🏷️ 自定义标签创建和管理
//! - 📝 标签名称和描述编辑
//! - 🎨 标签颜色和样式设置
//! - 📊 标签使用统计分析
//! - 🔄 标签状态管理控制
//!
//! ## 标签绑定
//! - 🔗 群组标签绑定关系管理
//! - 📋 批量标签绑定操作
//! - 🔍 标签绑定查询检索
//! - 📊 绑定关系统计分析
//! - 🔄 标签绑定状态同步
//!
//! ## 群组分类
//! - 📂 群组分类体系构建
//! - 🔍 按标签筛选群组
//! - 📊 群组标签分布统计
//! - 🎯 精准群组定位查找
//! - 📈 分类效果分析评估
//!
//! ## 权限控制
//! - 🔐 标签管理权限控制
//! - 👑 标签操作角色管理
//! - 📊 标签操作日志记录
//! - 🛡️ 标签数据安全保护
//! - 🔒 敏感标签访问限制
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
//! // 获取企业标签服务
//! let tenant_tag = &client.tenant_tag;
//!
//! // 创建标签
//! // let tag_request = CreateTagRequest::builder()
//! //     .name("重要项目")
//! //     .description("重要项目相关群组")
//! //     .color("#FF6B6B")
//! //     .build();
//! // let tag = tenant_tag.tag.create(tag_request, None).await?;
//!
//! // 绑定标签到群组
//! // let binding_request = CreateTagBindingRequest::builder()
//! //     .tag_id("tag_123")
//! //     .chat_id("chat_456")
//! //     .build();
//! // tenant_tag.tag_binding.create(binding_request, None).await?;
//!
//! // 查询群组标签
//! // let query_request = ListTagBindingRequest::builder()
//! //     .chat_id("chat_456")
//! //     .build();
//! // let bindings = tenant_tag.tag_binding.list(query_request, None).await?;
//!
//! // 按标签筛选群组
//! // let filter_request = FilterChatsByTagRequest::builder()
//! //     .tag_ids(vec!["tag_123", "tag_789"])
//! //     .operator("AND")
//! //     .build();
//! // let chats = tenant_tag.tag_binding.filter_chats(filter_request, None).await?;
//!
//! // 更新标签信息
//! // let update_request = UpdateTagRequest::builder()
//! //     .tag_id("tag_123")
//! //     .name("核心项目")
//! //     .description("公司核心项目群组")
//! //     .build();
//! // tenant_tag.tag.update(update_request, None).await?;
//! ```
//!
//! # 标签化管理特性
//!
//! - 🏷️ 灵活的标签体系设计
//! - 📊 直观的可视化管理界面
//! - 🔍 强大的搜索筛选能力
//! - 📈 详细的使用统计分析
//! - 🔄 高效的批量操作支持
//!
//! # 企业应用
//!
//! - 🏢 企业群组分类管理
//! - 📊 项目群组标识区分
//! - 🎯 精准群组定位查找
//! - 📋 群组使用统计分析
//! - 🔄 群组管理流程优化

use crate::core::config::Config;

pub mod models;
pub mod tag;
pub mod tag_binding;

use tag::TagService;
use tag_binding::TagBindingService;

/// 企业自定义群标签服务
pub struct TenantTagService {
    /// 标签管理服务
    pub tag: TagService,
    /// 标签绑定服务  
    pub tag_binding: TagBindingService,
}

impl TenantTagService {
    pub fn new(config: Config) -> Self {
        Self {
            tag: TagService::new(config.clone()),
            tag_binding: TagBindingService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_tenant_tag_service_creation() {
        let config = Config::default();
        let service = TenantTagService::new(config.clone());

        assert_eq!(service.tag.config.app_id, config.app_id);
        assert_eq!(service.tag.config.app_secret, config.app_secret);
        assert_eq!(service.tag_binding.config.app_id, config.app_id);
        assert_eq!(service.tag_binding.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_tenant_tag_service_with_custom_config() {
        let config = Config::builder()
            .app_id("tenant_tag_app")
            .app_secret("tenant_tag_secret")
            .req_timeout(Duration::from_secs(120))
            .build();

        let service = TenantTagService::new(config.clone());

        assert_eq!(service.tag.config.app_id, "tenant_tag_app");
        assert_eq!(service.tag.config.app_secret, "tenant_tag_secret");
        assert_eq!(
            service.tag.config.req_timeout,
            Some(Duration::from_secs(120))
        );

        assert_eq!(service.tag_binding.config.app_id, "tenant_tag_app");
        assert_eq!(service.tag_binding.config.app_secret, "tenant_tag_secret");
        assert_eq!(
            service.tag_binding.config.req_timeout,
            Some(Duration::from_secs(120))
        );
    }

    #[test]
    fn test_tenant_tag_service_config_independence() {
        let config1 = Config::builder().app_id("app_1").build();

        let config2 = Config::builder().app_id("app_2").build();

        let service1 = TenantTagService::new(config1);
        let service2 = TenantTagService::new(config2);

        assert_eq!(service1.tag.config.app_id, "app_1");
        assert_eq!(service1.tag_binding.config.app_id, "app_1");
        assert_eq!(service2.tag.config.app_id, "app_2");
        assert_eq!(service2.tag_binding.config.app_id, "app_2");
        assert_ne!(service1.tag.config.app_id, service2.tag.config.app_id);
    }

    #[test]
    fn test_tenant_tag_service_sub_services_access() {
        let config = Config::default();
        let service = TenantTagService::new(config.clone());

        // Test that we can access both sub-services
        assert_eq!(service.tag.config.app_id, config.app_id);
        assert_eq!(service.tag_binding.config.app_id, config.app_id);
    }

    #[test]
    fn test_tenant_tag_service_multiple_instances() {
        let config = Config::default();

        let service1 = TenantTagService::new(config.clone());
        let service2 = TenantTagService::new(config.clone());

        assert_eq!(service1.tag.config.app_id, service2.tag.config.app_id);
        assert_eq!(
            service1.tag_binding.config.app_id,
            service2.tag_binding.config.app_id
        );
    }

    #[test]
    fn test_tenant_tag_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = TenantTagService::new(config.clone());

        // Both services should have the same config values but be independent instances
        assert_eq!(service.tag.config.app_id, service.tag_binding.config.app_id);
        assert_eq!(
            service.tag.config.app_secret,
            service.tag_binding.config.app_secret
        );

        // But they should be independent instances (this tests that clone() worked)
        assert_eq!(service.tag.config.app_id, "clone_test_app");
        assert_eq!(service.tag_binding.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_tenant_tag_service_config_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(90))
            .build();

        let service = TenantTagService::new(config);

        assert_eq!(
            service.tag.config.req_timeout,
            Some(Duration::from_secs(90))
        );
        assert_eq!(
            service.tag_binding.config.req_timeout,
            Some(Duration::from_secs(90))
        );
    }
}
