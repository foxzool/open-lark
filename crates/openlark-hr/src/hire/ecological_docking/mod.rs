pub mod background_check;
pub mod exam;

use openlark_core::config::Config;

use background_check::BackgroundCheckService;
use exam::ExamService;

/// 生态对接服务
///
/// 提供与第三方生态系统的对接功能，包括背调服务和
/// 笔试服务的集成，支持企业与外部供应商的系统对接。
pub struct EcologicalDockingService {
    /// 背调服务
    pub background_check: BackgroundCheckService,
    /// 笔试服务
    pub exam: ExamService,
}

impl EcologicalDockingService {
    pub fn new(config: Config) -> Self {
        Self {
            background_check: BackgroundCheckService::new(config.clone()),
            exam: ExamService::new(config),
        }
    }
}
