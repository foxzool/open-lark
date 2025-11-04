// Integrations 服务模块

pub mod application;
pub mod bot;
pub mod cardkit;
pub mod directory;
pub mod task;
pub mod tenant;
pub mod vc;

/// Integrations 服务包
///
/// 提供飞书平台的集成功能服务，包括：
/// - 🔗 **应用管理**: 企业应用生命周期和配置管理
/// - 🤖 **机器人服务**: 自定义机器人和自动化助手
/// - 🎴 **卡片工具**: 交互式卡片和消息组件
/// - 🏢 **租户管理**: 多租户环境和权限管理
/// - 📋 **任务管理**: 项目任务和协作工作流
/// - 📇 **目录服务**: 组织架构和人员目录
/// - 📹 **视频会议**: 实时会议和协作工具
///
/// 为企业提供强大的第三方集成能力，支持系统间无缝数据交换和业务流程自动化。

#[derive(Debug)]
pub struct IntegrationsService {
    config: open_lark_core::core::config::Config,
}

impl IntegrationsService {
    pub fn new(config: open_lark_core::core::config::Config) -> Self {
        Self { config }
    }
}

impl open_lark_core::core::trait_system::Service for IntegrationsService {
    fn config(&self) -> &open_lark_core::core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "integrations"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use open_lark_core::core::*;
}
