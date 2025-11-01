pub mod agency;
pub mod external_system;
pub mod referral;
pub mod website;
use crate::core::config::Config;
use agency::AgencyService;
use external_system::ExternalSystemService;
use referral::ReferralService;
use website::WebsiteService;
/// 获取候选人服务
///
/// 提供从各种渠道获取候选人的功能，包括内推渠道、
/// 官网投递、猎头推荐和外部系统对接等服务。
pub struct GetCandidatesService {
}

impl GetCandidatesService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}