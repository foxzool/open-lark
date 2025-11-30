//! OpenLark Bitable 多维表格服务
//!
//! 提供飞书多维表格相关的API服务，包括：
//! - 表格管理：创建、删除、查询数据表
//! - 记录管理：增删改查表格记录
//! - 视图管理：创建和管理表格视图
//! - 字段管理：字段类型和属性管理
//! - 权限管理：角色和协作者权限
//! - 工作流管理：自动化流程配置
//! - 表单管理：表单问题管理
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use openlark_docs::bitable::BitableService;
//! use openlark_core::config::Config;
//!
//! let config = Config::default();
//! let bitable_service = BitableService::new(config);
//!
//! // 创建多维表格
//! let app = bitable_service.v1().app.create_builder()
//!     .name("我的表格")
//!     .build()
//!     .await?;
//!
//! // 添加数据表
//! let table = bitable_service.v1().app_table.create_builder()
//!     .app_token(&app.app_token)
//!     .name("数据表1")
//!     .build()
//!     .await?;
//! ```

pub mod v1;
pub mod v2;

pub use v1::*;

use openlark_core::{
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    SDKResult,
};
pub use openlark_core::prelude::*;

/// 多维表格服务
#[derive(Debug, Clone)]
pub struct BitableService {
    pub config: Config,
    pub transport: Transport,
}

impl BitableService {
    /// 创建新的多维表格服务实例
    pub fn new(config: Config) -> Self {
        Self {
            transport: Transport::new(),
            config,
        }
    }

    /// 获取V1版本API服务
    pub fn v1(&self) -> v1::BitableV1Service {
        v1::BitableV1Service::new(self.config.clone())
    }

    /// 获取V2版本API服务
    pub fn v2(&self) -> v2::BitableV2Service {
        v2::BitableV2Service::new(self.config.clone())
    }

    /// 刷新访问令牌
    pub async fn refresh_token(&self) -> SDKResult<()> {
        self.transport.refresh_token().await
    }

    /// 检查是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.transport.is_authenticated()
    }

    /// 获取访问令牌类型
    pub fn token_type(&self) -> AccessTokenType {
        self.transport.token_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitable_service_creation() {
        let config = openlark_core::config::Config::default();
        let bitable_service = BitableService::new(config);

        // 测试服务创建
        assert!(!bitable_service.config.app_id.is_empty());
        println!("BitableService 创建测试通过");
    }

    #[test]
    fn test_version_access() {
        let config = openlark_core::config::Config::default();
        let bitable_service = BitableService::new(config);

        // 测试版本访问
        let _v1_service = bitable_service.v1();
        let _v2_service = bitable_service.v2();

        println!("版本访问测试通过");
    }
}