/// OpenLark Bitable V1 多维表格服务模块
///
/// 提供飞书多维表格V1版本API服务，包括：
/// - 应用管理：创建、复制、更新多维表格
/// - 数据表管理：增删改查数据表
/// - 记录管理：表格记录的CRUD操作
/// - 视图管理：表格视图的创建和管理
/// - 字段管理：表格字段类型和属性
/// - 角色管理：自定义角色和协作者权限
/// - 工作流管理：自动化流程配置
use openlark_core::config::Config;

// 应用级别API
pub mod app;

// 应用子模块API
pub use app::*;

/// Bitable V1 服务
pub struct BitableV1Service {
    pub config: Config,
}

impl BitableV1Service {
    /// 创建Bitable V1服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取应用管理服务
    pub fn app(&self) -> app::AppService {
        app::AppService::new(self.config.clone())
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}
