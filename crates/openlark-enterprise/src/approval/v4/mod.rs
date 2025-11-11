// Approval v4 服务模块

pub mod approval;
pub mod external_approval;
pub mod external_instance;
pub mod external_task;
pub mod file;
pub mod instance;
pub mod instance_comment;
pub mod message;
pub mod p2_approval_instance_approved_v4;
pub mod p2_approval_instance_created_v4;
pub mod p2_approval_instance_rejected_v4;
pub mod search;
pub mod task;

/// Approval 服务
///
/// 提供完整的审批流程管理解决方案，包括：
/// - 📋 **审批定义**: 审批单、审批模板、审批流程等
/// - 🔗 **审批实例**: 审批单的生命周期管理和状态跟踪
/// - 🎯 **外部审批**: 与外部系统集成的审批流程
/// - 📁 **审批历史**: 完整的审批记录和统计分析
/// - 🔄 **审批自动化**: 支持自动化审批规则和条件判断
///
/// 为企业提供灵活的审批管理平台，支持复杂的企业审批流程和多系统集成。

#[derive(Debug)]
pub struct ApprovalService {
    config: openlark_core::config::Config,
}

impl ApprovalService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for ApprovalService {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "approval"
    }

    fn service_version() -> &'static str {
        "v4"
    }
}
