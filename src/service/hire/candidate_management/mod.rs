#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
pub mod application;
pub mod interview;
pub mod offer;
pub mod talent;
pub mod talent_pool;
use config::Config;
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
}

impl CandidateManagementService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}