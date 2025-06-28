pub mod models;
pub mod okr;
pub mod period;
pub mod period_rule;
pub mod progress_record;
pub mod review;

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
    pub fn new(config: Config) -> Self {
        Self {
            period: PeriodService::new(config.clone()),
            period_rule: PeriodRuleService::new(config.clone()),
            okr: OkrContentService::new(config.clone()),
            progress_record: ProgressRecordService::new(config.clone()),
            review: ReviewService::new(config),
        }
    }
}
