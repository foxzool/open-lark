pub mod agency;
pub mod external_system;
pub mod referral;
pub mod website;

use openlark_core::config::Config;

use agency::AgencyService;
use external_system::ExternalSystemService;
use referral::ReferralService;
use website::WebsiteService;

/// 获取候选人服务
///
/// 提供从各种渠道获取候选人的功能，包括内推渠道、
/// 官网投递、猎头推荐和外部系统对接等服务。
pub struct GetCandidatesService {
    /// 内推服务
    pub referral: ReferralService,
    /// 官网服务
    pub website: WebsiteService,
    /// 猎头服务
    pub agency: AgencyService,
    /// 外部系统服务
    pub external_system: ExternalSystemService,
}

impl GetCandidatesService {
    pub fn new(config: Config) -> Self {
        Self {
            referral: ReferralService::new(config.clone()),
            website: WebsiteService::new(config.clone()),
            agency: AgencyService::new(config.clone()),
            external_system: ExternalSystemService::new(config),
        }
    }
}
