//! 招聘（Hire）服务
//!
//! 提供飞书招聘平台的完整功能集，涵盖职位管理、候选人管理、面试流程、
//! Offer管理等全生命周期的招聘业务。这是企业人才获取和招聘管理的核心服务模块。
//!
//! # 核心功能模块
//!
//! ## 招聘配置 (recruitment_config)
//! - 🏢 组织架构和权限管理
//! - 📋 职位和招聘需求管理
//! - 🔄 招聘流程配置
//! - 📊 面试评价和Offer设置
//!
//! ## 候选人获取 (get_candidates)
//! - 🌐 招聘官网和渠道管理
//! - 👥 内推和推荐管理
//! - 🤝 猎头供应商对接
//! - 🔗 外部系统集成
//!
//! ## 候选人管理 (candidate_management)
//! - 👤 人才库管理和搜索
//! - 📄 简历投递和筛选
//! - 🎯 面试安排和评估
//! - 💼 Offer创建和管理
//! - 🎓 入职流程和状态跟踪
//!
//! ## 生态对接 (ecological_docking)
//! - 🔧 自定义字段管理
//! - 🔍 背调服务集成
//! - 📝 笔试平台对接
//! - 📊 第三方数据同步
//!
//! ## 内推账户 (referral_account)
//! - 💰 内推奖励和账户管理
//! - 💸 提现和财务对账
//! - 📈 内推效果统计
//!
//! ## 附件管理 (attachment)
//! - 📎 简历和文档上传
//! - 🔄 文件格式转换
//! - 🔗 附件链接管理
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取招聘服务
//! let hire = &client.hire;
//!
//! // 职位管理示例
//! // let job_request = CreateJobRequest::builder()
//! //     .title("高级软件工程师")
//! //     .department_id("dept_123")
//! //     .build();
//! // let job = hire.recruitment_config.job.create(job_request, None).await?;
//!
//! // 候选人管理示例
//! // let talent_request = CreateTalentRequest::builder()
//! //     .name("张三")
//! //     .email("zhangsan@example.com")
//! //     .build();
//! // let talent = hire.candidate_management.talent.create(talent_request, None).await?;
//! ```
//!
//! # 权限要求
//!
//! 使用招聘服务需要相应的应用权限：
//! - `hire:job` - 职位管理权限
//! - `hire:candidate` - 候选人管理权限
//! - `hire:application` - 投递管理权限
//! - `hire:interview` - 面试管理权限
//! - `hire:offer` - Offer管理权限
//! - `hire:onboard` - 入职管理权限

/// 附件管理功能
pub mod attachment;
/// 候选人管理功能
pub mod candidate_management;
/// 生态对接功能
pub mod ecological_docking;
/// 候选人获取功能
pub mod get_candidates;
/// 数据模型定义
pub mod models;
/// 招聘配置功能
pub mod recruitment_config;
/// 内推账户功能
pub mod referral_account;
/// 招聘服务 v1 版本
pub mod v1;

use std::sync::Arc;

use crate::core::config::Config;

use attachment::AttachmentService;
use candidate_management::CandidateManagementService;
use ecological_docking::EcologicalDockingService;
use get_candidates::GetCandidatesService;
use recruitment_config::RecruitmentConfigService;
use referral_account::ReferralAccountService;

/// 招聘服务
///
/// 企业招聘管理的统一入口，提供完整的人才获取和招聘流程管理能力。
/// 支持从职位发布到入职完成的全生命周期招聘业务。
///
/// # 服务架构
///
/// - **recruitment_config**: 招聘配置和基础设置
/// - **get_candidates**: 候选人获取和渠道管理
/// - **candidate_management**: 候选人全生命周期管理
/// - **ecological_docking**: 第三方系统集成
/// - **referral_account**: 内推奖励和账户管理
/// - **attachment**: 文档和附件处理
///
/// # 核心特性
///
/// - 🚀 完整的招聘流程管理
/// - 👥 多渠道候选人获取
/// - 🎯 智能简历筛选和匹配
/// - 📊 数据驱动的招聘分析
/// - 🔗 丰富的第三方集成能力
/// - 💰 内推激励和管理体系
///
/// # 适用场景
///
/// - 企业人才招聘和获取
/// - 招聘流程标准化管理
/// - 多渠道人才库建设
/// - 招聘数据分析和优化
/// - HR系统集成和协作
///
/// # 最佳实践
///
/// - 建立清晰的招聘流程和标准
/// - 充分利用多渠道候选人获取
/// - 及时更新职位和候选人状态
/// - 重视候选人体验和反馈
/// - 定期分析招聘数据和效果
pub struct HireService {
    /// 招聘配置服务 - 管理职位、流程、权限等基础配置
    pub recruitment_config: RecruitmentConfigService,
    /// 候选人获取服务 - 管理招聘渠道和候选人来源
    pub get_candidates: GetCandidatesService,
    /// 候选人管理服务 - 处理候选人全生命周期操作
    pub candidate_management: CandidateManagementService,
    /// 生态对接服务 - 集成第三方背调、笔试等服务
    pub ecological_docking: EcologicalDockingService,
    /// 内推账户服务 - 管理内推奖励和账户系统
    pub referral_account: ReferralAccountService,
    /// 附件服务 - 处理简历、文档等附件管理
    pub attachment: AttachmentService,
}

impl HireService {
    /// 创建新的招聘服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的招聘服务实例，包含所有子服务模块
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            recruitment_config: RecruitmentConfigService::new(Arc::clone(&config)),
            get_candidates: GetCandidatesService::new(Arc::clone(&config)),
            candidate_management: CandidateManagementService::new(Arc::clone(&config)),
            ecological_docking: EcologicalDockingService::new(Arc::clone(&config)),
            referral_account: ReferralAccountService::new(Arc::clone(&config)),
            attachment: AttachmentService::new(config),
        }
    }
}
