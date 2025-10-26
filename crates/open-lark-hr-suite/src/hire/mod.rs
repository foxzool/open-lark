// Hire 服务模块

pub mod v1;

/// Hire 服务
///
/// 提供完整的招聘管理功能，包括：
/// - 🎯 **招聘流程管理**: 从申请到入职的全流程跟踪
/// - 📋 **人才管理**: 候选人信息管理和评估
/// - 👥 **录用管理**: offer 状态和录用流程管理
/// - 📊 **招聘分析**: 招聘数据统计和分析报告
///
/// 为企业提供完整的招聘解决方案，支持多种招聘渠道和流程管理。

#[derive(Debug)]
pub struct HireService {
    config: open_lark_core::core::config::Config,
}

impl HireService {
    pub fn new(config: open_lark_core::core::config::Config) -> Self {
        Self { config }
    }
}

impl open_lark_core::core::trait_system::Service for HireService {
    fn config(&self) -> &open_lark_core::core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "hire"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
