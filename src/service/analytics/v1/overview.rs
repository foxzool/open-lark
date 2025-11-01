//! 数据概览服务
//!
//! 提供企业数据的整体概览和核心指标统计：
//! - 企业基础数据统计
//! - 用户活跃度概览
//! - 应用使用情况概览
//! - 关键业务指标监控
//! - 数据趋势和变化分析

use crate::core::config::Config;
use crate::service::analytics::v1::*;
use crate::service::payroll::v1::datasource::IssueSeverity;
use chrono::{DateTime, Utc};
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 数据概览服务
#[derive(Debug, Clone)]
pub struct OverviewService {
    config: Config,
}

impl OverviewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 企业数据概览 ====================

    /// 获取企业概览数据
    pub async fn get_enterprise_overview(
        &self,
        request: &GetEnterpriseOverviewRequest,
    ) -> SDKResult<GetEnterpriseOverviewResponse> {
        // 模拟企业概览数据
        Ok(GetEnterpriseOverviewResponse {
            enterprise_info: EnterpriseInfo {
                tenant_id: "tenant_001".to_string(),
                enterprise_name: "示例企业".to_string(),
                industry: "互联网".to_string(),
                scale: EnterpriseScale::Medium,
                employee_count: 1250,
                department_count: 28,
                active_user_count: 1180,
                created_at: Some(chrono::Utc::now() - chrono::Duration::days(365)),
            },
            core_metrics: CoreMetrics {
                total_users: 1250,
                active_users: 1180,
                daily_active_users: 980,
                weekly_active_users: 1120,
                monthly_active_users: 1180,
                total_departments: 28,
                total_applications: 15,
                active_applications: 13,
                total_messages: 156780,
                total_files: 28945,
                total_meetings: 3421,
                total_tasks: 8756,
            },
            growth_metrics: GrowthMetrics {
                user_growth_rate: 12.5,
                department_growth_rate: 8.3,
                application_growth_rate: 15.7,
                message_growth_rate: 23.4,
                file_growth_rate: 18.9,
                meeting_growth_rate: 9.8,
                task_growth_rate: 14.2,
                comparison_period: request.period.clone(),
            },
            usage_trends: vec![
                UsageTrend {
                    metric_name: "日活用户数".to_string(),
                    current_period: UsageTrendData {
                        total: 980,
                        average_daily: 980,
                        peak_value: 1156,
                        peak_date: Some("2024-10-28".to_string()),
                        trend_direction: TrendDirection::Up,
                        change_percentage: 8.5,
                    },
                    previous_period: Some(UsageTrendData {
                        total: 903,
                        average_daily: 903,
                        peak_value: 1089,
                        peak_date: Some("2024-09-30".to_string()),
                        trend_direction: TrendDirection::Up,
                        change_percentage: 6.2,
                    }),
                },
                UsageTrend {
                    metric_name: "消息发送量".to_string(),
                    current_period: UsageTrendData {
                        total: 156780,
                        average_daily: 5226,
                        peak_value: 7890,
                        peak_date: Some("2024-10-28".to_string()),
                        trend_direction: TrendDirection::Up,
                        change_percentage: 23.4,
                    },
                    previous_period: Some(UsageTrendData {
                        total: 127092,
                        average_daily: 4236,
                        peak_value: 6234,
                        peak_date: Some("2024-09-30".to_string()),
                        trend_direction: TrendDirection::Up,
                        change_percentage: 18.7,
                    }),
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取用户活跃度概览
    pub async fn get_user_activity_overview(
        &self,
        request: &GetUserActivityOverviewRequest,
    ) -> SDKResult<GetUserActivityOverviewResponse> {
        Ok(GetUserActivityOverviewResponse {
            summary: UserActivitySummary {
                total_users: 1250,
                active_users: 1180,
                inactive_users: 70,
                new_users: 45,
                churned_users: 12,
                activation_rate: 94.4,
                retention_rate: 98.9,
                period: request.period.clone(),
            },
            activity_distribution: vec![
                ActivityDistribution {
                    activity_level: ActivityLevel::HighlyActive,
                    user_count: 487,
                    percentage: 41.3,
                    description: "每日活跃".to_string(),
                },
                ActivityDistribution {
                    activity_level: ActivityLevel::ModeratelyActive,
                    user_count: 492,
                    percentage: 41.7,
                    description: "每周活跃".to_string(),
                },
                ActivityDistribution {
                    activity_level: ActivityLevel::LowActive,
                    user_count: 201,
                    percentage: 17.0,
                    description: "偶尔活跃".to_string(),
                },
                ActivityDistribution {
                    activity_level: ActivityLevel::Inactive,
                    user_count: 70,
                    percentage: 5.9,
                    description: "长期不活跃".to_string(),
                },
            ],
            daily_activity: self.generate_daily_activity_data(&request.period),
            department_activity: vec![
                DepartmentActivity {
                    department_id: "dept_001".to_string(),
                    department_name: "技术研发部".to_string(),
                    total_users: 450,
                    active_users: 438,
                    activation_rate: 97.3,
                    average_messages_per_user: 28.5,
                    average_files_per_user: 6.2,
                },
                DepartmentActivity {
                    department_id: "dept_002".to_string(),
                    department_name: "市场营销部".to_string(),
                    total_users: 180,
                    active_users: 168,
                    activation_rate: 93.3,
                    average_messages_per_user: 45.8,
                    average_files_per_user: 12.1,
                },
                DepartmentActivity {
                    department_id: "dept_003".to_string(),
                    department_name: "人力资源部".to_string(),
                    total_users: 65,
                    active_users: 64,
                    activation_rate: 98.5,
                    average_messages_per_user: 32.4,
                    average_files_per_user: 8.7,
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取应用使用概览
    pub async fn get_app_usage_overview(
        &self,
        request: &GetAppUsageOverviewRequest,
    ) -> SDKResult<GetAppUsageOverviewResponse> {
        Ok(GetAppUsageOverviewResponse {
            summary: AppUsageSummary {
                total_applications: 15,
                active_applications: 13,
                inactive_applications: 2,
                new_applications: 3,
                discontinued_applications: 1,
                total_sessions: 45678,
                average_session_duration: 8.5, // 分钟
                total_interactions: 234567,
                period: request.period.clone(),
            },
            top_applications: vec![
                ApplicationUsage {
                    app_id: "app_im".to_string(),
                    app_name: "即时通讯".to_string(),
                    category: "沟通协作".to_string(),
                    total_sessions: 15678,
                    unique_users: 1180,
                    average_session_duration: 12.3,
                    total_interactions: 89012,
                    user_satisfaction: 4.2,
                    usage_rank: 1,
                },
                ApplicationUsage {
                    app_id: "app_docs".to_string(),
                    app_name: "云文档".to_string(),
                    category: "协作工具".to_string(),
                    total_sessions: 12456,
                    unique_users: 980,
                    average_session_duration: 18.7,
                    total_interactions: 56734,
                    user_satisfaction: 4.5,
                    usage_rank: 2,
                },
                ApplicationUsage {
                    app_id: "app_calendar".to_string(),
                    app_name: "日历".to_string(),
                    category: "效率工具".to_string(),
                    total_sessions: 8901,
                    unique_users: 750,
                    average_session_duration: 6.2,
                    total_interactions: 23456,
                    user_satisfaction: 4.1,
                    usage_rank: 3,
                },
                ApplicationUsage {
                    app_id: "app_tasks".to_string(),
                    app_name: "任务管理".to_string(),
                    category: "项目管理".to_string(),
                    total_sessions: 5643,
                    unique_users: 420,
                    average_session_duration: 15.8,
                    total_interactions: 34567,
                    user_satisfaction: 3.9,
                    usage_rank: 4,
                },
            ],
            category_breakdown: vec![
                CategoryUsage {
                    category: "沟通协作".to_string(),
                    application_count: 4,
                    total_sessions: 28945,
                    unique_users: 1180,
                    usage_percentage: 63.4,
                },
                CategoryUsage {
                    category: "协作工具".to_string(),
                    application_count: 3,
                    total_sessions: 12456,
                    unique_users: 980,
                    usage_percentage: 27.3,
                },
                CategoryUsage {
                    category: "效率工具".to_string(),
                    application_count: 5,
                    total_sessions: 4277,
                    unique_users: 650,
                    usage_percentage: 9.3,
                },
                CategoryUsage {
                    category: "项目管理".to_string(),
                    application_count: 1,
                    total_sessions: 0,
                    unique_users: 0,
                    usage_percentage: 0.0,
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取关键业务指标
    pub async fn get_key_business_metrics(
        &self,
        request: &GetKeyBusinessMetricsRequest,
    ) -> SDKResult<GetKeyBusinessMetricsResponse> {
        Ok(GetKeyBusinessMetricsResponse {
            metrics: vec![
                BusinessMetric {
                    metric_id: "metric_001".to_string(),
                    metric_name: "员工协作效率".to_string(),
                    category: "协作指标".to_string(),
                    current_value: 85.6,
                    target_value: 90.0,
                    unit: Some("分数".to_string()),
                    trend_direction: TrendDirection::Up,
                    change_percentage: 5.2,
                    status: MetricStatus::Good,
                    description: Some("基于消息发送、文件共享和协作频率的综合评分".to_string()),
                },
                BusinessMetric {
                    metric_id: "metric_002".to_string(),
                    metric_name: "会议效率".to_string(),
                    category: "会议指标".to_string(),
                    current_value: 78.3,
                    target_value: 85.0,
                    unit: Some("分数".to_string()),
                    trend_direction: TrendDirection::Stable,
                    change_percentage: 0.8,
                    status: MetricStatus::Warning,
                    description: Some("基于会议时长、参与度和行动完成率的评分".to_string()),
                },
                BusinessMetric {
                    metric_id: "metric_003".to_string(),
                    metric_name: "任务完成率".to_string(),
                    category: "项目指标".to_string(),
                    current_value: 92.1,
                    target_value: 95.0,
                    unit: Some("%".to_string()),
                    trend_direction: TrendDirection::Up,
                    change_percentage: 3.7,
                    status: MetricStatus::Good,
                    description: Some("按时完成的任务比例".to_string()),
                },
                BusinessMetric {
                    metric_id: "metric_004".to_string(),
                    metric_name: "知识沉淀率".to_string(),
                    category: "知识指标".to_string(),
                    current_value: 68.9,
                    target_value: 80.0,
                    unit: Some("%".to_string()),
                    trend_direction: TrendDirection::Up,
                    change_percentage: 8.5,
                    status: MetricStatus::Warning,
                    description: Some("文档创建与知识分享的活跃度".to_string()),
                },
            ],
            period: request.period.clone(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取数据质量概览
    pub async fn get_data_quality_overview(
        &self,
        request: &GetDataQualityOverviewRequest,
    ) -> SDKResult<GetDataQualityOverviewResponse> {
        Ok(GetDataQualityOverviewResponse {
            overall_score: 92.3,
            quality_dimensions: vec![
                QualityDimension {
                    dimension_name: "完整性".to_string(),
                    score: 95.2,
                    weight: 0.25,
                    description: Some("数据字段的完整性评分".to_string()),
                    issues: vec!["部分用户档案缺少部门信息".to_string()],
                },
                QualityDimension {
                    dimension_name: "准确性".to_string(),
                    score: 91.8,
                    weight: 0.30,
                    description: Some("数据准确性和一致性评分".to_string()),
                    issues: vec!["少量重复数据需要清理".to_string()],
                },
                QualityDimension {
                    dimension_name: "及时性".to_string(),
                    score: 89.6,
                    weight: 0.20,
                    description: Some("数据更新的及时性评分".to_string()),
                    issues: vec!["部分统计数据延迟24小时".to_string()],
                },
                QualityDimension {
                    dimension_name: "一致性".to_string(),
                    score: 93.1,
                    weight: 0.25,
                    description: Some("跨系统数据一致性评分".to_string()),
                    issues: vec![],
                },
            ],
            recent_issues: vec![DataQualityIssue {
                issue_id: "dq_001".to_string(),
                severity: IssueSeverity::Low,
                category: "数据完整性".to_string(),
                description: "5%的用户缺少部门信息".to_string(),
                affected_records: 62,
                detected_at: chrono::Utc::now() - chrono::Duration::hours(2),
                suggested_action: "联系HR部门补充用户部门信息".to_string(),
            }],
            quality_trends: vec![
                QualityTrend {
                    date: "2024-10-28".to_string(),
                    overall_score: 92.3,
                    completeness_score: 95.2,
                    accuracy_score: 91.8,
                    timeliness_score: 89.6,
                    consistency_score: 93.1,
                },
                QualityTrend {
                    date: "2024-10-27".to_string(),
                    overall_score: 91.8,
                    completeness_score: 94.9,
                    accuracy_score: 91.5,
                    timeliness_score: 89.2,
                    consistency_score: 92.8,
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 辅助方法 ====================

    fn generate_daily_activity_data(&self, period: &str) -> Vec<DailyActivityData> {
        let mut data = Vec::new();
        let base_date = chrono::Utc::now();

        for i in 0..30 {
            let date = base_date - chrono::Duration::days(i);
            let base_value = 900.0 + (i as f64 * 2.5);
            let variation = (i as f64 * 0.8).sin() * 50.0;

            data.push(DailyActivityData {
                date: date.format("%Y-%m-%d").to_string(),
                active_users: (base_value + variation) as i32,
                total_messages: ((base_value + variation) * 5.2) as i32,
                total_files: ((base_value + variation) * 1.8) as i32,
                total_meetings: ((base_value + variation) * 0.15) as i32,
            });
        }

        data.reverse();
        data
    }
}

// ==================== 请求数据模型 ====================

/// 获取企业概览请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEnterpriseOverviewRequest {
    /// 统计周期
    pub period: String,
    /// 是否包含详细信息
    pub include_details: Option<bool>,
    /// 维度过滤
    pub dimension_filters: Option<Vec<String>>,
}

/// 获取用户活跃度概览请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserActivityOverviewRequest {
    /// 统计周期
    pub period: String,
    /// 部门过滤
    pub department_filter: Option<String>,
    /// 用户类型过滤
    pub user_type_filter: Option<String>,
}

/// 获取应用使用概览请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppUsageOverviewRequest {
    /// 统计周期
    pub period: String,
    /// 应用分类过滤
    pub category_filter: Option<String>,
    /// 是否包含详细统计
    pub include_detailed_stats: Option<bool>,
}

/// 获取关键业务指标请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKeyBusinessMetricsRequest {
    /// 统计周期
    pub period: String,
    /// 指标分类过滤
    pub category_filter: Option<String>,
    /// 状态过滤
    pub status_filter: Option<MetricStatus>,
}

/// 获取数据质量概览请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataQualityOverviewRequest {
    /// 统计周期
    pub period: String,
    /// 质量维度过滤
    pub dimension_filter: Option<String>,
    /// 严重程度过滤
    pub severity_filter: Option<IssueSeverity>,
}

// ==================== 响应数据模型 ====================

/// 获取企业概览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEnterpriseOverviewResponse {
    /// 企业信息
    pub enterprise_info: EnterpriseInfo,
    /// 核心指标
    pub core_metrics: CoreMetrics,
    /// 增长指标
    pub growth_metrics: GrowthMetrics,
    /// 使用趋势
    pub usage_trends: Vec<UsageTrend>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取用户活跃度概览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserActivityOverviewResponse {
    /// 活跃度汇总
    pub summary: UserActivitySummary,
    /// 活跃度分布
    pub activity_distribution: Vec<ActivityDistribution>,
    /// 日活跃度数据
    pub daily_activity: Vec<DailyActivityData>,
    /// 部门活跃度
    pub department_activity: Vec<DepartmentActivity>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取应用使用概览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppUsageOverviewResponse {
    /// 使用汇总
    pub summary: AppUsageSummary,
    /// 热门应用
    pub top_applications: Vec<ApplicationUsage>,
    /// 分类统计
    pub category_breakdown: Vec<CategoryUsage>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取关键业务指标响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKeyBusinessMetricsResponse {
    /// 业务指标列表
    pub metrics: Vec<BusinessMetric>,
    /// 统计周期
    pub period: String,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取数据质量概览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataQualityOverviewResponse {
    /// 总体质量评分
    pub overall_score: f64,
    /// 质量维度
    pub quality_dimensions: Vec<QualityDimension>,
    /// 最近问题
    pub recent_issues: Vec<DataQualityIssue>,
    /// 质量趋势
    pub quality_trends: Vec<QualityTrend>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 企业信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseInfo {
    /// 租户ID
    pub tenant_id: String,
    /// 企业名称
    pub enterprise_name: String,
    /// 所属行业
    pub industry: String,
    /// 企业规模
    pub scale: EnterpriseScale,
    /// 员工数量
    pub employee_count: i32,
    /// 部门数量
    pub department_count: i32,
    /// 活跃用户数量
    pub active_user_count: i32,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 企业规模
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseScale {
    /// 小型
    Small,
    /// 中型
    Medium,
    /// 大型
    Large,
    /// 超大型
    Enterprise,
}

/// 核心指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMetrics {
    /// 总用户数
    pub total_users: i32,
    /// 活跃用户数
    pub active_users: i32,
    /// 日活用户数
    pub daily_active_users: i32,
    /// 周活用户数
    pub weekly_active_users: i32,
    /// 月活用户数
    pub monthly_active_users: i32,
    /// 总部门数
    pub total_departments: i32,
    /// 总应用数
    pub total_applications: i32,
    /// 活跃应用数
    pub active_applications: i32,
    /// 总消息数
    pub total_messages: i64,
    /// 总文件数
    pub total_files: i64,
    /// 总会议数
    pub total_meetings: i64,
    /// 总任务数
    pub total_tasks: i64,
}

/// 增长指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    /// 用户增长率
    pub user_growth_rate: f64,
    /// 部门增长率
    pub department_growth_rate: f64,
    /// 应用增长率
    pub application_growth_rate: f64,
    /// 消息增长率
    pub message_growth_rate: f64,
    /// 文件增长率
    pub file_growth_rate: f64,
    /// 会议增长率
    pub meeting_growth_rate: f64,
    /// 任务增长率
    pub task_growth_rate: f64,
    /// 对比周期
    pub comparison_period: String,
}

/// 使用趋势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTrend {
    /// 指标名称
    pub metric_name: String,
    /// 当前周期数据
    pub current_period: UsageTrendData,
    /// 上一周期数据
    pub previous_period: Option<UsageTrendData>,
}

/// 使用趋势数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTrendData {
    /// 总计
    pub total: i64,
    /// 日均值
    pub average_daily: f64,
    /// 峰值
    pub peak_value: i64,
    /// 峰值日期
    pub peak_date: Option<String>,
    /// 趋势方向
    pub trend_direction: TrendDirection,
    /// 变化百分比
    pub change_percentage: f64,
}

/// 用户活跃度汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivitySummary {
    /// 总用户数
    pub total_users: i32,
    /// 活跃用户数
    pub active_users: i32,
    /// 不活跃用户数
    pub inactive_users: i32,
    /// 新用户数
    pub new_users: i32,
    /// 流失用户数
    pub churned_users: i32,
    /// 激活率
    pub activation_rate: f64,
    /// 留存率
    pub retention_rate: f64,
    /// 统计周期
    pub period: String,
}

/// 活跃度分布
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDistribution {
    /// 活跃度级别
    pub activity_level: ActivityLevel,
    /// 用户数量
    pub user_count: i32,
    /// 百分比
    pub percentage: f64,
    /// 描述
    pub description: String,
}

/// 活跃度级别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityLevel {
    /// 高活跃
    HighlyActive,
    /// 中等活跃
    ModeratelyActive,
    /// 低活跃
    LowActive,
    /// 不活跃
    Inactive,
}

/// 日活跃度数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyActivityData {
    /// 日期
    pub date: String,
    /// 活跃用户数
    pub active_users: i32,
    /// 总消息数
    pub total_messages: i32,
    /// 总文件数
    pub total_files: i32,
    /// 总会议数
    pub total_meetings: i32,
}

/// 部门活跃度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentActivity {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 总用户数
    pub total_users: i32,
    /// 活跃用户数
    pub active_users: i32,
    /// 激活率
    pub activation_rate: f64,
    /// 人均消息数
    pub average_messages_per_user: f64,
    /// 人均文件数
    pub average_files_per_user: f64,
}

/// 应用使用汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsageSummary {
    /// 总应用数
    pub total_applications: i32,
    /// 活跃应用数
    pub active_applications: i32,
    /// 不活跃应用数
    pub inactive_applications: i32,
    /// 新应用数
    pub new_applications: i32,
    /// 停用应用数
    pub discontinued_applications: i32,
    /// 总会话数
    pub total_sessions: i64,
    /// 平均会话时长(分钟)
    pub average_session_duration: f64,
    /// 总交互数
    pub total_interactions: i64,
    /// 统计周期
    pub period: String,
}

/// 应用使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationUsage {
    /// 应用ID
    pub app_id: String,
    /// 应用名称
    pub app_name: String,
    /// 应用分类
    pub category: String,
    /// 总会话数
    pub total_sessions: i64,
    /// 独立用户数
    pub unique_users: i32,
    /// 平均会话时长(分钟)
    pub average_session_duration: f64,
    /// 总交互数
    pub total_interactions: i64,
    /// 用户满意度
    pub user_satisfaction: f64,
    /// 使用排名
    pub usage_rank: i32,
}

/// 分类使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryUsage {
    /// 分类名称
    pub category: String,
    /// 应用数量
    pub application_count: i32,
    /// 总会话数
    pub total_sessions: i64,
    /// 独立用户数
    pub unique_users: i32,
    /// 使用占比
    pub usage_percentage: f64,
}

/// 业务指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetric {
    /// 指标ID
    pub metric_id: String,
    /// 指标名称
    pub metric_name: String,
    /// 指标分类
    pub category: String,
    /// 当前值
    pub current_value: f64,
    /// 目标值
    pub target_value: f64,
    /// 单位
    pub unit: Option<String>,
    /// 趋势方向
    pub trend_direction: TrendDirection,
    /// 变化百分比
    pub change_percentage: f64,
    /// 状态
    pub status: MetricStatus,
    /// 描述
    pub description: Option<String>,
}

/// 指标状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricStatus {
    /// 优秀
    Excellent,
    /// 良好
    Good,
    /// 警告
    Warning,
    /// 危险
    Critical,
}

/// 质量维度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDimension {
    /// 维度名称
    pub dimension_name: String,
    /// 评分
    pub score: f64,
    /// 权重
    pub weight: f64,
    /// 描述
    pub description: Option<String>,
    /// 发现的问题
    pub issues: Vec<String>,
}

/// 数据质量问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityIssue {
    /// 问题ID
    pub issue_id: String,
    /// 严重程度
    pub severity: IssueSeverity,
    /// 问题分类
    pub category: String,
    /// 问题描述
    pub description: String,
    /// 影响记录数
    pub affected_records: i64,
    /// 发现时间
    pub detected_at: DateTime<Utc>,
    /// 建议操作
    pub suggested_action: String,
}

/// 质量趋势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrend {
    /// 日期
    pub date: String,
    /// 总体评分
    pub overall_score: f64,
    /// 完整性评分
    pub completeness_score: f64,
    /// 准确性评分
    pub accuracy_score: f64,
    /// 及时性评分
    pub timeliness_score: f64,
    /// 一致性评分
    pub consistency_score: f64,
}

// 实现Default trait
impl Default for GetEnterpriseOverviewRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            include_details: Some(true),
            dimension_filters: None,
        }
    }
}

impl Default for GetUserActivityOverviewRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            department_filter: None,
            user_type_filter: None,
        }
    }
}

impl Default for GetAppUsageOverviewRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            category_filter: None,
            include_detailed_stats: Some(true),
        }
    }
}

impl Default for GetKeyBusinessMetricsRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            category_filter: None,
            status_filter: None,
        }
    }
}

impl Default for GetDataQualityOverviewRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            dimension_filter: None,
            severity_filter: None,
        }
    }
}
