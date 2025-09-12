//! OKR服务
//!
//! 提供飞书OKR的完整功能集，支持目标管理、关键结果跟踪、进展记录、
//! 周期管理、复盘总结等企业级目标管理能力。是企业战略执行和绩效管理的核心工具。
//!
//! # 核心功能
//!
//! ## 目标管理
//! - 🎯 目标(Objective)创建和管理
//! - 📊 关键结果(Key Result)设定
//! - 🔗 目标层级关联关系
//! - 📈 目标进度跟踪
//! - 🎭 目标状态管理
//!
//! ## 周期管理
//! - 📅 OKR周期创建和配置
//! - ⏰ 周期时间范围设置
//! - 📋 周期状态管理
//! - 🔄 周期规则配置
//! - 📊 周期数据统计
//!
//! ## 进展记录
//! - 📝 进展更新记录创建
//! - 📊 进度百分比跟踪
//! - 📷 进展附件管理
//! - 💬 进展评论和讨论
//! - 📈 进展趋势分析
//!
//! ## 复盘管理
//! - 🔍 周期复盘和总结
//! - 📊 成果评估和分析
//! - 💡 经验教训提取
//! - 📈 改进建议生成
//! - 📋 复盘报告管理
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
//! // 获取OKR服务
//! let okr = &client.okr;
//!
//! // 获取OKR周期列表
//! // let period_request = ListPeriodsRequest::builder()
//! //     .page_size(20)
//! //     .status("active")
//! //     .build();
//! // let periods = okr.period.list(period_request, None).await?;
//!
//! // 获取用户OKR列表
//! // let user_okr_request = ListUserOkrRequest::builder()
//! //     .user_id("user_123")
//! //     .period_id("period_456")
//! //     .build();
//! // let user_okrs = okr.okr.list_user_okr(user_okr_request, None).await?;
//!
//! // 创建进展记录
//! // let progress_request = CreateProgressRecordRequest::builder()
//! //     .kr_id("kr_789")
//! //     .content("本月完成了核心功能开发，进度达到80%")
//! //     .progress_rate(80)
//! //     .build();
//! // okr.progress_record.create(progress_request, None).await?;
//!
//! // 获取复盘信息
//! // let review_request = GetReviewRequest::builder()
//! //     .period_id("period_456")
//! //     .user_id("user_123")
//! //     .build();
//! // let review = okr.review.get(review_request, None).await?;
//! ```
//!
//! # 目标管理特性
//!
//! - 🎯 SMART目标设定原则
//! - 📊 量化指标跟踪
//! - 🔗 目标对齐和联动
//! - 📈 实时进度可视化
//! - 🏆 成就里程碑管理
//!
//! # 企业应用
//!
//! - 🏢 公司战略目标分解
//! - 👥 团队目标协同管理
//! - 📈 个人绩效提升
//! - 📊 数据驱动决策
//! - 🔄 持续改进文化

/// 数据模型定义
pub mod models;
/// OKR内容服务
#[allow(clippy::module_inception)]
pub mod okr;
/// 周期管理功能
pub mod period;
/// 周期规则功能
pub mod period_rule;
/// 进展记录功能
pub mod progress_record;
/// 复盘管理功能
pub mod review;

use std::sync::Arc;

use crate::core::config::Config;

use okr::OkrContentService;
use period::PeriodService;
use period_rule::PeriodRuleService;
use progress_record::ProgressRecordService;
use review::ReviewService;

/// 飞书 OKR 服务
///
/// 飞书 OKR 为企业提供了完整的目标管理和绩效跟踪功能，涵盖从目标制定、
/// 进展跟踪到复盘总结的完整流程。本服务封装了相关API接口，支持：
///
/// ## 主要功能
///
/// ### OKR 周期管理
/// - **周期创建**: 创建新的 OKR 周期，设置时间范围和规则
/// - **状态管理**: 修改 OKR 周期状态（草稿、进行中、已结束等）
/// - **周期查询**: 获取 OKR 周期列表，支持筛选和分页
///
/// ### 周期规则管理
/// - **规则查询**: 获取 OKR 周期的配置规则和模板
/// - **规则配置**: 支持不同部门和角色的 OKR 规则设置
///
/// ### OKR 内容管理
/// - **用户 OKR**: 获取指定用户的 OKR 列表
/// - **批量查询**: 支持批量获取多个用户的 OKR 信息
/// - **目标跟踪**: 跟踪 Objective 和 Key Result 的进展情况
///
/// ### 进展记录管理
/// - **记录创建**: 创建 OKR 进展更新记录
/// - **记录管理**: 支持更新、删除和查询进展记录
/// - **附件支持**: 上传图片等附件到进展记录
/// - **进度跟踪**: 记录和展示 OKR 完成进度
///
/// ### 复盘管理
/// - **复盘查询**: 查询 OKR 周期的复盘信息
/// - **成果总结**: 支持总结和评估 OKR 执行效果
///
/// ## 使用场景
///
/// - **企业目标管理**: 设置和跟踪公司、部门、个人目标
/// - **绩效评估**: 基于 OKR 完成情况进行绩效考核
/// - **进展跟踪**: 定期更新和监控目标执行进度
/// - **复盘改进**: 周期性总结经验，持续改进目标管理
/// - **数据分析**: 通过 OKR 数据分析组织目标达成情况
///
/// ## 权限要求
///
/// 使用本服务需要相应的应用权限：
/// - `okr:okr`: OKR 基本权限
/// - `okr:okr:readonly`: OKR 只读权限
/// - `okr:progress_record`: 进展记录管理权限
///
/// ## 示例用法
///
/// ```ignore
/// use open_lark::prelude::*;
/// use open_lark::service::okr::models::*;
///
/// // 创建客户端
/// let client = LarkClient::builder(app_id, app_secret)
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 获取 OKR 周期列表
/// let period_request = PeriodListRequest {
///     page_size: Some(20),
///     page_token: None,
///     status: Some(PeriodStatus::Active),
///     ..Default::default()
/// };
///
/// let periods = client.okr.period.list_periods(period_request, None).await?;
///
/// // 获取用户 OKR 列表
/// let okr_request = OkrListRequest {
///     user_id: "user_123".to_string(),
///     period_id: "period_456".to_string(),
///     ..Default::default()
/// };
///
/// let okrs = client.okr.okr.list_user_okrs(okr_request, None).await?;
///
/// // 创建进展记录
/// let progress_request = ProgressRecordCreateRequest {
///     okr_id: "okr_789".to_string(),
///     content: "本周完成了关键功能开发，进度达到80%".to_string(),
///     progress_rate: Some(80),
///     ..Default::default()
/// };
///
/// let progress = client.okr.progress_record.create_progress_record(progress_request, None).await?;
/// ```
pub struct OkrService {
    /// 周期管理服务
    pub period: PeriodService,
    /// 周期规则服务
    pub period_rule: PeriodRuleService,
    /// OKR 内容服务
    pub okr: OkrContentService,
    /// 进展记录服务
    pub progress_record: ProgressRecordService,
    /// 复盘服务
    pub review: ReviewService,
}

impl OkrService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            period: PeriodService::new(Arc::clone(&config)),
            period_rule: PeriodRuleService::new(Arc::clone(&config)),
            okr: OkrContentService::new(Arc::clone(&config)),
            progress_record: ProgressRecordService::new(Arc::clone(&config)),
            review: ReviewService::new(config),
        }
    }
}
