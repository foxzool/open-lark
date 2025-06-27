pub mod attachment;
pub mod candidate_management;
pub mod ecological_docking;
pub mod get_candidates;
pub mod models;
pub mod recruitment_config;
pub mod referral_account;

use crate::core::config::Config;

use attachment::AttachmentService;
use candidate_management::CandidateManagementService;
use ecological_docking::EcologicalDockingService;
use get_candidates::GetCandidatesService;
use recruitment_config::RecruitmentConfigService;
use referral_account::ReferralAccountService;

/// 飞书招聘服务
///
/// 飞书招聘为企业提供了完整的招聘管理和人才获取功能，涵盖从职位发布、
/// 候选人管理到入职全流程的招聘业务。本服务封装了相关API接口，支持：
///
/// ## 主要功能
///
/// ### 招聘相关配置
/// - **地址管理**: 查询地点列表、获取地址信息
/// - **权限管理**: 角色详情、角色列表、用户角色管理
/// - **职位管理**: 职位创建、更新、配置、发布、关闭等全生命周期管理
/// - **招聘需求**: 创建、更新、查询招聘需求和模板
/// - **招聘流程**: 获取招聘流程信息和配置
/// - **项目管理**: 项目列表和配置
/// - **面试设置**: 面试评价表、轮次类型、面试官管理
/// - **Offer设置**: Offer申请表配置和管理
///
/// ### 获取候选人
/// - **内推管理**: 内推信息查询、官网职位广告管理
/// - **官网管理**: 招聘官网推广渠道、用户管理、投递管理
/// - **猎头管理**: 猎头供应商、保护期设置、账户管理
/// - **外部系统**: 外部人才、投递、面试、Offer、背调信息集成
///
/// ### 候选人管理
/// - **人才库**: 人才加入/移除、人才库搜索
/// - **人才管理**: 人才创建、更新、标签操作、文件夹管理
/// - **投递流程**: 投递管理、阶段转移、终止恢复
/// - **评估管理**: 简历评估、笔试、面试
/// - **Offer管理**: Offer创建、更新、状态管理
/// - **入职管理**: 入职操作、状态更新、e-HR集成
/// - **备注管理**: 备注创建、更新、查询
///
/// ### 生态对接
/// - **账号管理**: 生态账号自定义字段管理
/// - **背调集成**: 背调自定义字段、套餐管理、订单处理
/// - **笔试集成**: 试卷管理、笔试安排、结果回传
///
/// ### 内推账户
/// - **账户管理**: 注册、启用、停用内推账户
/// - **资产管理**: 查询账户余额、提现操作
/// - **数据对账**: 提现数据对账
///
/// ### 附件管理
/// - **附件操作**: 创建附件、获取附件信息
/// - **文件处理**: PDF格式下载链接
///
/// ## 使用场景
///
/// - **企业招聘**: 完整的企业招聘流程管理
/// - **人才获取**: 多渠道人才获取和管理
/// - **招聘流程**: 标准化招聘流程和评估体系
/// - **数据分析**: 招聘数据统计和分析
/// - **系统集成**: 与第三方HR系统、背调、笔试平台集成
///
/// ## 权限要求
///
/// 使用本服务需要相应的应用权限：
/// - `hire:job`: 职位管理权限
/// - `hire:candidate`: 候选人管理权限
/// - `hire:application`: 投递管理权限
/// - `hire:interview`: 面试管理权限
/// - `hire:offer`: Offer管理权限
/// - `hire:onboard`: 入职管理权限
///
/// ## 示例用法
///
/// ```ignore
/// use open_lark::prelude::*;
/// use open_lark::service::hire::models::*;
///
/// // 创建客户端
/// let client = LarkClient::builder(app_id, app_secret)
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 获取职位列表
/// let job_request = JobListRequest {
///     page_size: Some(50),
///     page_token: None,
///     status: Some("active".to_string()),
///     ..Default::default()
/// };
///
/// let jobs = client.hire.recruitment_config.job.list_jobs(job_request, None).await?;
///
/// // 创建人才
/// let talent_request = TalentCreateRequest {
///     name: "张三".to_string(),
///     email: Some("zhangsan@example.com".to_string()),
///     phone: Some("13800138000".to_string()),
///     ..Default::default()
/// };
///
/// let talent = client.hire.candidate_management.talent.create_talent(talent_request, None).await?;
///
/// // 创建投递
/// let application_request = ApplicationCreateRequest {
///     talent_id: talent.data.unwrap().talent_id,
///     job_id: "job_123".to_string(),
///     stage_id: "stage_456".to_string(),
///     ..Default::default()
/// };
///
/// let application = client.hire.candidate_management.application.create_application(application_request, None).await?;
/// ```
pub struct HireService {
    /// 招聘相关配置服务
    pub recruitment_config: RecruitmentConfigService,
    /// 获取候选人服务
    pub get_candidates: GetCandidatesService,
    /// 候选人管理服务
    pub candidate_management: CandidateManagementService,
    /// 生态对接服务
    pub ecological_docking: EcologicalDockingService,
    /// 内推账户服务
    pub referral_account: ReferralAccountService,
    /// 附件服务
    pub attachment: AttachmentService,
}

impl HireService {
    pub fn new(config: Config) -> Self {
        Self {
            recruitment_config: RecruitmentConfigService::new(config.clone()),
            get_candidates: GetCandidatesService::new(config.clone()),
            candidate_management: CandidateManagementService::new(config.clone()),
            ecological_docking: EcologicalDockingService::new(config.clone()),
            referral_account: ReferralAccountService::new(config.clone()),
            attachment: AttachmentService::new(config),
        }
    }
}
