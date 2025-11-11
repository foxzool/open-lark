pub mod application;
pub mod auth;
pub mod interview_settings;
pub mod job;
pub mod job_process;
pub mod job_requirement;
pub mod location;
pub mod offer_settings;
pub mod subject;

use openlark_core::{config::Config, trait_system::Service};

use application::ApplicationConfigService;
use auth::AuthService;
use interview_settings::InterviewSettingsService;
use job::JobService;
use job_process::JobProcessService;
use job_requirement::JobRequirementService;
use location::LocationService;
use offer_settings::OfferSettingsService;
use subject::SubjectService;

/// 招聘相关配置服务
///
/// 提供招聘系统的基础配置功能，包括地址管理、权限设置、
/// 职位管理、招聘需求、流程配置、候选人配置等核心配置服务。
pub struct RecruitmentConfigService {
    /// 地址服务
    pub location: LocationService,
    /// 权限服务
    pub auth: AuthService,
    /// 职位服务
    pub job: JobService,
    /// 招聘需求服务
    pub job_requirement: JobRequirementService,
    /// 招聘流程服务
    pub job_process: JobProcessService,
    /// 项目服务
    pub subject: SubjectService,
    /// 候选人配置服务
    pub application: ApplicationConfigService,
    /// 面试设置服务
    pub interview_settings: InterviewSettingsService,
    /// Offer设置服务
    pub offer_settings: OfferSettingsService,
}

impl RecruitmentConfigService {
    pub fn new(config: Config) -> Self {
        Self {
            location: LocationService::new(config.clone()),
            auth: AuthService::new(config.clone()),
            job: JobService::new(config.clone()),
            job_requirement: JobRequirementService::new(config.clone()),
            job_process: JobProcessService::new(config.clone()),
            subject: SubjectService::new(config.clone()),
            application: ApplicationConfigService::new(config.clone()),
            interview_settings: InterviewSettingsService::new(config.clone()),
            offer_settings: OfferSettingsService::new(config),
        }
    }
}

impl Service for RecruitmentConfigService {
    fn config(&self) -> &Config {
        &self.location.config
    }

    fn service_name() -> &'static str {
        "recruitment_config"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
