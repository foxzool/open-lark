#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Contact服务模块
//!
//! 提供飞书开放平台通讯录服务的完整实现，包括：
//! - 用户管理：创建、更新、删除、查询用户
//! - 部门管理：组织架构的维护和查询
//! - 用户组管理：用户组的创建和管理
//! - 自定义字段：企业自定义用户字段管理
//! - 职位信息：职位体系的管理

use crate::config::Config;

// 数据模型 - 暂时禁用以修复语法错误
// pub mod models;

// V3 版本服务
#[cfg(feature = "contact")]
pub mod v3;

/// Contact服务
#[derive(Debug, Clone)]
pub struct ContactService {
    pub config: Config,
    /// V3版本服务
    #[cfg(feature = "contact")]
    pub v3: ContactServiceV3,
}

#[cfg(feature = "contact")]
#[derive(Debug, Clone)]
pub struct ContactServiceV3 {
    pub config: Config,
    /// 自定义字段服务
    #[cfg(feature = "contact")]
    pub custom_attr: crate::service::contact::v3::custom_attr::CustomAttrService,
}

impl ContactService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "contact")]
            v3: ContactServiceV3::new(config),
        }
    }
}

#[cfg(feature = "contact")]
impl ContactServiceV3 {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "contact")]
            custom_attr: crate::service::contact::v3::custom_attr::CustomAttrService::new(config),
        }
    }
}
