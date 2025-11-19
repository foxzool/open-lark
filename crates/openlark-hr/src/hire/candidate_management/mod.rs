pub mod application;
pub mod interview;
pub mod offer;
pub mod talent;
pub mod talent_pool;

use openlark_core::config::Config;

use application::ApplicationService;
use interview::InterviewService;
use offer::OfferService;
use talent::TalentService;
use talent_pool::TalentPoolService;

/// 候选人管理服务
///
/// 提供候选人相关的核心管理功能，包括人才库管理、
/// 人才档案管理、投递流程管理、面试管理和 Offer 管理等服务。
pub struct CandidateManagementService {
    /// 人才库服务
    pub talent_pool: TalentPoolService,
    /// 人才服务
    pub talent: TalentService,
    /// 投递服务
    pub application: ApplicationService,
    /// 面试服务
    pub interview: InterviewService,
    /// Offer服务
    pub offer: OfferService,
}

impl CandidateManagementService {
    pub fn new(config: Config) -> Self {
        Self {
            talent_pool: TalentPoolService::new(config.clone()),
            talent: TalentService::new(config.clone()),
            application: ApplicationService::new(config.clone()),
            interview: InterviewService::new(config.clone()),
            offer: OfferService::new(config),
        }
    }
}
