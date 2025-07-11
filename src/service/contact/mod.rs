//! 通讯录（Contact）服务
//!
//! 提供企业通讯录的完整管理功能，支持用户、部门、组织架构等核心要素的
//! 生命周期管理。这是企业人员和组织管理的核心服务模块。
//!
//! # 核心功能
//!
//! ## 用户管理
//! - 👤 用户信息的增删改查
//! - 🔄 用户状态和生命周期管理
//! - 📧 用户身份验证和邮箱管理
//! - 🏷️ 用户标签和分组管理
//!
//! ## 组织架构
//! - 🏢 部门层级结构管理
//! - 👥 部门成员和负责人设置
//! - 📊 组织架构调整和优化
//! - 🔄 部门合并和拆分操作
//!
//! ## 权限和角色
//! - 🔐 权限范围管理和控制
//! - 👑 用户组和角色分配
//! - 🎯 精细化权限控制
//! - 🔒 安全策略和访问控制
//!
//! ## 职级职务
//! - 🎖️ 职级体系管理
//! - 💼 职务分配和调整
//! - 📈 晋升和调岗流程
//! - 📋 职位描述和要求
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
//! // 获取通讯录服务
//! let contact = &client.contact;
//!
//! // 使用v3版本API
//! // let users = contact.v3.user.list(request, None).await?;
//! // let departments = contact.v3.department.list(request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v3版本，提供最新的功能和最佳性能。v3版本包含：
//! - 增强的用户管理功能
//! - 灵活的组织架构操作
//! - 完善的权限控制机制
//! - 高效的批量操作支持

/// 通讯录数据模型定义
pub mod models;
/// 通讯录服务 v3 版本
pub mod v3;

pub use models::*;
pub use v3::*;

use crate::core::config::Config;

/// 通讯录服务
///
/// 企业通讯录的统一管理入口，提供完整的人员和组织管理功能。
/// 支持企业级的用户生命周期管理、组织架构调整和权限控制。
///
/// # 服务架构
///
/// - **v3**: 最新版本API，提供完整功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🚀 高性能批量操作
/// - 🔐 企业级安全控制
/// - 📊 灵活的组织架构
/// - 🎯 精细化权限管理
/// - 🔄 完整的生命周期支持
///
/// # 适用场景
///
/// - 企业人力资源管理
/// - 组织架构调整和优化
/// - 权限和访问控制
/// - 用户身份管理
/// - 通讯录同步和集成
///
/// # 最佳实践
///
/// - 定期同步和更新用户信息
/// - 合理设计组织架构层级
/// - 遵循最小权限原则
/// - 建立完善的审计机制
pub struct ContactService {
    /// v3版本API服务
    pub v3: v3::V3,
}

impl ContactService {
    /// 创建新的通讯录服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的通讯录服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v3: v3::V3::new(config),
        }
    }
}
