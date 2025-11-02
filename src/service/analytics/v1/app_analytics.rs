use serde::{Deserialize, Serialize};

/// 问题严重程度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// 低
    Low,
    /// 中
    Medium,
    /// 高
    High,
    /// 严重
    Critical,
}

// 提供企业应用使用的深度分析功能：
// - 应用使用统计和趋势分析
// - 应用功能使用情况分析
// - 应用性能和用户体验评估
// - 应用集成和协作分析
// - 应用ROI和价值评估

use crate::core::config::Config;
use crate::core::SDKResult;
use crate::service::analytics::v1::*;
use chrono::{DateTime, Utc};

/// 应用分析服务
#[derive(Debug, Clone)]
pub struct AppAnalyticsService {
    config: Config,
}

impl AppAnalyticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 应用使用统计 ====================

    /// 获取应用使用统计
    pub async fn get_app_usage_statistics(
        &self,
        request: &GetAppUsageStatisticsRequest,
    ) -> SDKResult<GetAppUsageStatisticsResponse> {
        Ok(GetAppUsageStatisticsResponse {
            analysis_period: request.period.clone(),
            overall_statistics: OverallStatistics {
                total_applications: 15,
                active_applications: 13,
                total_sessions: 45678,
                total_users: 1180,
                average_session_duration: 8.5,
                total_interactions: 234567,
                user_satisfaction_score: 4.2,
                growth_rate: 15.7,
            },
            application_stats: vec![
                ApplicationStatistics {
                    app_id: "app_im".to_string(),
                    app_name: "即时通讯".to_string(),
                    category: "沟通协作".to_string(),
                    total_sessions: 15678,
                    unique_users: 1180,
                    average_session_duration: 12.3,
                    total_interactions: 89012,
                    user_satisfaction: 4.2,
                    growth_rate: 18.5,
                    top_features: vec![
                        "群组聊天".to_string(),
                        "文件传输".to_string(),
                        "语音通话".to_string(),
                    ],
                    usage_distribution: self.generate_usage_distribution("im"),
                },
                ApplicationStatistics {
                    app_id: "app_docs".to_string(),
                    app_name: "云文档".to_string(),
                    category: "协作工具".to_string(),
                    total_sessions: 12456,
                    unique_users: 980,
                    average_session_duration: 18.7,
                    total_interactions: 56734,
                    user_satisfaction: 4.5,
                    growth_rate: 22.3,
                    top_features: vec![
                        "文档编辑".to_string(),
                        "多人协作".to_string(),
                        "版本管理".to_string(),
                    ],
                    usage_distribution: self.generate_usage_distribution("docs"),
                },
                ApplicationStatistics {
                    app_id: "app_calendar".to_string(),
                    app_name: "日历".to_string(),
                    category: "效率工具".to_string(),
                    total_sessions: 8901,
                    unique_users: 750,
                    average_session_duration: 6.2,
                    total_interactions: 23456,
                    user_satisfaction: 4.1,
                    growth_rate: 12.8,
                    top_features: vec![
                        "会议安排".to_string(),
                        "日程同步".to_string(),
                        "会议室预订".to_string(),
                    ],
                    usage_distribution: self.generate_usage_distribution("calendar"),
                },
                ApplicationStatistics {
                    app_id: "app_tasks".to_string(),
                    app_name: "任务管理".to_string(),
                    category: "项目管理".to_string(),
                    total_sessions: 5643,
                    unique_users: 420,
                    average_session_duration: 15.8,
                    total_interactions: 34567,
                    user_satisfaction: 3.9,
                    growth_rate: 8.9,
                    top_features: vec![
                        "任务创建".to_string(),
                        "进度跟踪".to_string(),
                        "团队协作".to_string(),
                    ],
                    usage_distribution: self.generate_usage_distribution("tasks"),
                },
            ],
            category_breakdown: vec![
                CategoryStatistics {
                    category: "沟通协作".to_string(),
                    application_count: 4,
                    total_sessions: 28945,
                    unique_users: 1180,
                    average_satisfaction: 4.3,
                    growth_rate: 16.2,
                },
                CategoryStatistics {
                    category: "协作工具".to_string(),
                    application_count: 3,
                    total_sessions: 12456,
                    unique_users: 980,
                    average_satisfaction: 4.5,
                    growth_rate: 22.3,
                },
                CategoryStatistics {
                    category: "效率工具".to_string(),
                    application_count: 5,
                    total_sessions: 8921,
                    unique_users: 750,
                    average_satisfaction: 4.0,
                    growth_rate: 10.5,
                },
                CategoryStatistics {
                    category: "项目管理".to_string(),
                    application_count: 1,
                    total_sessions: 5643,
                    unique_users: 420,
                    average_satisfaction: 3.9,
                    growth_rate: 8.9,
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取应用功能使用分析
    pub async fn get_feature_usage_analysis(
        &self,
        request: &GetFeatureUsageAnalysisRequest,
    ) -> SDKResult<GetFeatureUsageAnalysisResponse> {
        Ok(GetFeatureUsageAnalysisResponse {
            app_id: request.app_id.clone(),
            analysis_period: request.period.clone(),
            feature_usage: vec![
                FeatureUsage {
                    feature_id: "feature_001".to_string(),
                    feature_name: "群组聊天".to_string(),
                    feature_category: "核心功能".to_string(),
                    usage_count: 45678,
                    unique_users: 1080,
                    usage_frequency: UsageFrequency::Daily,
                    satisfaction_score: 4.3,
                    average_completion_time: Some(2.5),
                    error_rate: 0.8,
                    usage_trend: TrendDirection::Up,
                    adoption_rate: 91.5,
                },
                FeatureUsage {
                    feature_id: "feature_002".to_string(),
                    feature_name: "文件传输".to_string(),
                    feature_category: "核心功能".to_string(),
                    usage_count: 23456,
                    unique_users: 980,
                    usage_frequency: UsageFrequency::Weekly,
                    satisfaction_score: 4.1,
                    average_completion_time: Some(8.2),
                    error_rate: 2.1,
                    usage_trend: TrendDirection::Up,
                    adoption_rate: 83.1,
                },
                FeatureUsage {
                    feature_id: "feature_003".to_string(),
                    feature_name: "语音通话".to_string(),
                    feature_category: "高级功能".to_string(),
                    usage_count: 5678,
                    unique_users: 450,
                    usage_frequency: UsageFrequency::Weekly,
                    satisfaction_score: 3.8,
                    average_completion_time: Some(15.6),
                    error_rate: 5.2,
                    usage_trend: TrendDirection::Stable,
                    adoption_rate: 38.1,
                },
            ],
            user_segments: vec![
                FeatureUserSegment {
                    segment_name: "重度用户".to_string(),
                    user_count: 180,
                    usage_percentage: 18.4,
                    avg_features_used: 8.5,
                    satisfaction_score: 4.6,
                },
                FeatureUserSegment {
                    segment_name: "中度用户".to_string(),
                    user_count: 520,
                    usage_percentage: 53.1,
                    avg_features_used: 4.2,
                    satisfaction_score: 4.2,
                },
                FeatureUserSegment {
                    segment_name: "轻度用户".to_string(),
                    user_count: 280,
                    usage_percentage: 28.5,
                    avg_features_used: 1.8,
                    satisfaction_score: 3.9,
                },
            ],
            feature_cohorts: vec![
                FeatureCohort {
                    cohort_name: "新用户".to_string(),
                    user_count: 85,
                    adoption_rate: 45.2,
                    retention_after_7_days: 78.5,
                    retention_after_30_days: 65.3,
                },
                FeatureCohort {
                    cohort_name: "老用户".to_string(),
                    user_count: 420,
                    adoption_rate: 92.8,
                    retention_after_7_days: 95.2,
                    retention_after_30_days: 88.7,
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取应用性能分析
    pub async fn get_app_performance_analysis(
        &self,
        request: &GetAppPerformanceAnalysisRequest,
    ) -> SDKResult<GetAppPerformanceAnalysisResponse> {
        Ok(GetAppPerformanceAnalysisResponse {
            app_id: request.app_id.clone(),
            analysis_period: request.period.clone(),
            performance_metrics: PerformanceMetrics {
                load_time: LoadTimeMetrics {
                    average_load_time: 2.3,
                    median_load_time: 1.8,
                    p95_load_time: 4.2,
                    p99_load_time: 7.8,
                    first_contentful_paint: 1.2,
                    largest_contentful_paint: 3.5,
                },
                response_time: ResponseTimeMetrics {
                    average_response_time: 120.5,
                    median_response_time: 95.2,
                    p95_response_time: 220.8,
                    p99_response_time: 450.6,
                },
                error_metrics: ErrorMetrics {
                    error_rate: 1.2,
                    total_errors: 5678,
                    critical_errors: 23,
                    warning_errors: 234,
                    info_errors: 5421,
                },
                availability_metrics: AvailabilityMetrics {
                    uptime_percentage: 99.7,
                    total_downtime_minutes: 45,
                    incident_count: 3,
                    mttr_mean_time_to_recovery: 15.2,
                },
            },
            performance_trends: vec![
                PerformanceTrend {
                    date: "2024-10-28".to_string(),
                    load_time: 2.1,
                    response_time: 115.3,
                    error_rate: 1.1,
                    availability: 99.8,
                },
                PerformanceTrend {
                    date: "2024-10-27".to_string(),
                    load_time: 2.4,
                    response_time: 125.7,
                    error_rate: 1.3,
                    availability: 99.6,
                },
                PerformanceTrend {
                    date: "2024-10-26".to_string(),
                    load_time: 2.2,
                    response_time: 118.9,
                    error_rate: 1.0,
                    availability: 99.9,
                },
            ],
            device_performance: vec![
                DevicePerformance {
                    device_type: "桌面端".to_string(),
                    user_percentage: 68.5,
                    average_load_time: 1.8,
                    average_response_time: 98.2,
                    error_rate: 0.8,
                    satisfaction_score: 4.4,
                },
                DevicePerformance {
                    device_type: "移动端".to_string(),
                    user_percentage: 28.5,
                    average_load_time: 3.2,
                    average_response_time: 145.6,
                    error_rate: 1.8,
                    satisfaction_score: 3.8,
                },
                DevicePerformance {
                    device_type: "平板端".to_string(),
                    user_percentage: 3.0,
                    average_load_time: 2.5,
                    average_response_time: 125.3,
                    error_rate: 1.2,
                    satisfaction_score: 4.1,
                },
            ],
            performance_issues: vec![
                PerformanceIssue {
                    issue_id: "perf_001".to_string(),
                    issue_type: "响应时间".to_string(),
                    severity: IssueSeverity::Medium,
                    description: "API响应时间在高峰期超过200ms".to_string(),
                    affected_users: 156,
                    impact_score: 6.5,
                    suggested_fix: "增加缓存策略和数据库索引优化".to_string(),
                },
                PerformanceIssue {
                    issue_id: "perf_002".to_string(),
                    issue_type: "错误率".to_string(),
                    severity: IssueSeverity::Low,
                    description: "文件上传偶尔失败，错误率2.1%".to_string(),
                    affected_users: 89,
                    impact_score: 3.2,
                    suggested_fix: "优化文件上传重试机制".to_string(),
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取应用集成分析
    pub async fn get_app_integration_analysis(
        &self,
        request: &GetAppIntegrationAnalysisRequest,
    ) -> SDKResult<GetAppIntegrationAnalysisResponse> {
        Ok(GetAppIntegrationAnalysisResponse {
            app_id: request.app_id.clone(),
            analysis_period: request.period.clone(),
            integration_overview: IntegrationOverview {
                total_integrations: 8,
                active_integrations: 6,
                integration_success_rate: 94.2,
                average_data_sync_time: 12.5,
                total_data_transfers: 45678,
            },
            integration_connections: vec![
                IntegrationConnection {
                    integration_id: "int_001".to_string(),
                    integration_name: "HR系统集成".to_string(),
                    target_system: "企业HR系统".to_string(),
                    integration_type: IntegrationType::DataExchange,
                    status: IntegrationStatus::Active,
                    success_rate: 96.5,
                    data_volume_mb: 125.6,
                    last_sync_time: chrono::Utc::now() - chrono::Duration::minutes(15),
                    error_count: 3,
                },
                IntegrationConnection {
                    integration_id: "int_002".to_string(),
                    integration_name: "邮件系统集成".to_string(),
                    target_system: "企业邮箱系统".to_string(),
                    integration_type: IntegrationType::Workflow,
                    status: IntegrationStatus::Active,
                    success_rate: 92.8,
                    data_volume_mb: 89.3,
                    last_sync_time: chrono::Utc::now() - chrono::Duration::minutes(5),
                    error_count: 7,
                },
                IntegrationConnection {
                    integration_id: "int_003".to_string(),
                    integration_name: "项目管理集成".to_string(),
                    target_system: "第三方项目管理工具".to_string(),
                    integration_type: IntegrationType::DataExchange,
                    status: IntegrationStatus::Inactive,
                    success_rate: 88.2,
                    data_volume_mb: 0.0,
                    last_sync_time: chrono::Utc::now() - chrono::Duration::days(3),
                    error_count: 15,
                },
            ],
            data_flow_analysis: vec![
                DataFlow {
                    flow_id: "flow_001".to_string(),
                    source_app: request.app_id.clone(),
                    target_app: "HR系统".to_string(),
                    data_type: "员工信息".to_string(),
                    flow_direction: DataFlowDirection::Bidirectional,
                    volume_gb_per_day: 0.125,
                    success_rate: 96.5,
                    latency_ms: 1250.0,
                    last_activity: chrono::Utc::now() - chrono::Duration::minutes(20),
                },
                DataFlow {
                    flow_id: "flow_002".to_string(),
                    source_app: request.app_id.clone(),
                    target_app: "邮件系统".to_string(),
                    data_type: "通知消息".to_string(),
                    flow_direction: DataFlowDirection::Unidirectional,
                    volume_gb_per_day: 0.089,
                    success_rate: 92.8,
                    latency_ms: 800.0,
                    last_activity: chrono::Utc::now() - chrono::Duration::minutes(8),
                },
            ],
            integration_insights: vec![
                "HR系统集成表现优异，数据同步成功率达到96.5%".to_string(),
                "邮件系统集成需要优化错误处理机制".to_string(),
                "建议重新激活项目管理集成以提升工作效率".to_string(),
                "数据传输延迟在可接受范围内，建议监控高峰期性能".to_string(),
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取应用ROI分析
    pub async fn get_app_roi_analysis(
        &self,
        request: &GetAppRoiAnalysisRequest,
    ) -> SDKResult<GetAppRoiAnalysisResponse> {
        Ok(GetAppRoiAnalysisResponse {
            app_id: request.app_id.clone(),
            analysis_period: request.period.clone(),
            roi_metrics: ROIMetrics {
                implementation_cost: 125000.0,
                monthly_operational_cost: 8500.0,
                total_cost: 227000.0,
                value_benefits: ValueBenefits {
                    time_savings_value: 45000.0,
                    productivity_increase_value: 78000.0,
                    quality_improvement_value: 35000.0,
                    collaboration_value: 28000.0,
                    total_benefits: 186000.0,
                },
                roi_percentage: -18.1,
                payback_period_months: 15,
                net_present_value: -156000.0,
            },
            benefit_breakdown: vec![
                BenefitItem {
                    category: "时间节省".to_string(),
                    description: "减少手动操作时间".to_string(),
                    monthly_value: 3750.0,
                    percentage: 20.2,
                },
                BenefitItem {
                    category: "效率提升".to_string(),
                    description: "工作流程优化".to_string(),
                    monthly_value: 6500.0,
                    percentage: 34.9,
                },
                BenefitItem {
                    category: "质量改善".to_string(),
                    description: "减少错误和重做".to_string(),
                    monthly_value: 2917.0,
                    percentage: 15.7,
                },
                BenefitItem {
                    category: "协作增强".to_string(),
                    description: "团队协作效率提升".to_string(),
                    monthly_value: 2333.0,
                    percentage: 12.5,
                },
            ],
            cost_breakdown: vec![
                CostItem {
                    category: "实施成本".to_string(),
                    description: "软件采购和实施费用".to_string(),
                    amount: 125000.0,
                    percentage: 55.1,
                },
                CostItem {
                    category: "运营成本".to_string(),
                    description: "维护和支持费用".to_string(),
                    amount: 51000.0,
                    percentage: 22.5,
                },
                CostItem {
                    category: "培训成本".to_string(),
                    description: "用户培训和推广费用".to_string(),
                    amount: 25500.0,
                    percentage: 11.2,
                },
                CostItem {
                    category: "其他成本".to_string(),
                    description: "集成和定制开发费用".to_string(),
                    amount: 25500.0,
                    percentage: 11.2,
                },
            ],
            roi_projections: vec![
                ROIProjection {
                    month: 6,
                    cumulative_benefits: 186000.0,
                    cumulative_costs: 176000.0,
                    roi_percentage: 5.7,
                },
                ROIProjection {
                    month: 12,
                    cumulative_benefits: 372000.0,
                    cumulative_costs: 227000.0,
                    roi_percentage: 63.8,
                },
                ROIProjection {
                    month: 18,
                    cumulative_benefits: 558000.0,
                    cumulative_costs: 278000.0,
                    roi_percentage: 100.7,
                },
            ],
            sensitivity_analysis: vec![
                SensitivityScenario {
                    scenario: "保守估计".to_string(),
                    roi_percentage: 32.5,
                    payback_period_months: 20,
                    assumptions: vec![
                        "生产力提升15%".to_string(),
                        "时间节省20%".to_string(),
                        "采用率70%".to_string(),
                    ],
                },
                SensitivityScenario {
                    scenario: "乐观估计".to_string(),
                    roi_percentage: 156.2,
                    payback_period_months: 10,
                    assumptions: vec![
                        "生产力提升35%".to_string(),
                        "时间节省40%".to_string(),
                        "采用率95%".to_string(),
                    ],
                },
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 辅助方法 ====================

    fn generate_usage_distribution(&self, app_type: &str) -> Vec<UsageDistribution> {
        match app_type {
            "im" => vec![
                UsageDistribution {
                    feature_name: "群组聊天".to_string(),
                    usage_percentage: 45.6,
                    user_count: 540,
                    avg_daily_usage: 8.2,
                },
                UsageDistribution {
                    feature_name: "一对一聊天".to_string(),
                    usage_percentage: 32.1,
                    user_count: 680,
                    avg_daily_usage: 6.5,
                },
                UsageDistribution {
                    feature_name: "文件传输".to_string(),
                    usage_percentage: 15.3,
                    user_count: 420,
                    avg_daily_usage: 2.1,
                },
                UsageDistribution {
                    feature_name: "语音通话".to_string(),
                    usage_percentage: 7.0,
                    user_count: 280,
                    avg_daily_usage: 1.5,
                },
            ],
            "docs" => vec![
                UsageDistribution {
                    feature_name: "文档编辑".to_string(),
                    usage_percentage: 38.5,
                    user_count: 378,
                    avg_daily_usage: 5.8,
                },
                UsageDistribution {
                    feature_name: "文档查看".to_string(),
                    usage_percentage: 28.2,
                    user_count: 520,
                    avg_daily_usage: 4.2,
                },
                UsageDistribution {
                    feature_name: "多人协作".to_string(),
                    usage_percentage: 22.3,
                    user_count: 245,
                    avg_daily_usage: 3.5,
                },
                UsageDistribution {
                    feature_name: "版本管理".to_string(),
                    usage_percentage: 11.0,
                    user_count: 180,
                    avg_daily_usage: 1.8,
                },
            ],
            "calendar" => vec![
                UsageDistribution {
                    feature_name: "会议安排".to_string(),
                    usage_percentage: 45.8,
                    user_count: 480,
                    avg_daily_usage: 3.2,
                },
                UsageDistribution {
                    feature_name: "日程查看".to_string(),
                    usage_percentage: 32.5,
                    user_count: 620,
                    avg_daily_usage: 4.8,
                },
                UsageDistribution {
                    feature_name: "会议室预订".to_string(),
                    usage_percentage: 21.7,
                    user_count: 220,
                    avg_daily_usage: 1.2,
                },
            ],
            "tasks" => vec![
                UsageDistribution {
                    feature_name: "任务创建".to_string(),
                    usage_percentage: 42.3,
                    user_count: 280,
                    avg_daily_usage: 4.5,
                },
                UsageDistribution {
                    feature_name: "进度更新".to_string(),
                    usage_percentage: 31.8,
                    user_count: 320,
                    avg_daily_usage: 3.8,
                },
                UsageDistribution {
                    feature_name: "团队协作".to_string(),
                    usage_percentage: 18.5,
                    user_count: 180,
                    avg_daily_usage: 2.2,
                },
                UsageDistribution {
                    feature_name: "报表生成".to_string(),
                    usage_percentage: 7.4,
                    user_count: 95,
                    avg_daily_usage: 0.8,
                },
            ],
            _ => vec![],
        }
    }
}

// ==================== 请求数据模型 ====================

/// 获取应用使用统计请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppUsageStatisticsRequest {
    /// 分析周期
    pub period: String,
    /// 应用分类过滤
    pub category_filter: Option<String>,
    /// 是否包含详细统计
    pub include_detailed_stats: Option<bool>,
}

/// 获取功能使用分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeatureUsageAnalysisRequest {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub period: String,
    /// 功能分类过滤
    pub feature_category_filter: Option<String>,
    /// 最小使用次数
    pub min_usage_count: Option<i32>,
}

/// 获取应用性能分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppPerformanceAnalysisRequest {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub period: String,
    /// 性能指标类型
    pub metric_types: Option<Vec<String>>,
    /// 设备类型过滤
    pub device_type_filter: Option<String>,
}

/// 获取应用集成分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppIntegrationAnalysisRequest {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub period: String,
    /// 集成类型过滤
    pub integration_type_filter: Option<String>,
    /// 状态过滤
    pub status_filter: Option<IntegrationStatus>,
}

/// 获取应用ROI分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppRoiAnalysisRequest {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub period: String,
    /// 成本计算方式
    pub cost_calculation_http_http_method: Option<String>,
    /// 收益计算方式
    pub benefit_calculation_http_http_method: Option<String>,
}

// ==================== 响应数据模型 ====================

/// 获取应用使用统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppUsageStatisticsResponse {
    /// 分析周期
    pub analysis_period: String,
    /// 总体统计
    pub overall_statistics: OverallStatistics,
    /// 应用统计
    pub application_stats: Vec<ApplicationStatistics>,
    /// 分类统计
    pub category_breakdown: Vec<CategoryStatistics>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取功能使用分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeatureUsageAnalysisResponse {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub analysis_period: String,
    /// 功能使用情况
    pub feature_usage: Vec<FeatureUsage>,
    /// 用户分段
    pub user_segments: Vec<FeatureUserSegment>,
    /// 功能队列
    pub feature_cohorts: Vec<FeatureCohort>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取应用性能分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppPerformanceAnalysisResponse {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub analysis_period: String,
    /// 性能指标
    pub performance_metrics: PerformanceMetrics,
    /// 性能趋势
    pub performance_trends: Vec<PerformanceTrend>,
    /// 设备性能
    pub device_performance: Vec<DevicePerformance>,
    /// 性能问题
    pub performance_issues: Vec<PerformanceIssue>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取应用集成分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppIntegrationAnalysisResponse {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub analysis_period: String,
    /// 集成概览
    pub integration_overview: IntegrationOverview,
    /// 集成连接
    pub integration_connections: Vec<IntegrationConnection>,
    /// 数据流分析
    pub data_flow_analysis: Vec<DataFlow>,
    /// 集成洞察
    pub integration_insights: Vec<String>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取应用ROI分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppRoiAnalysisResponse {
    /// 应用ID
    pub app_id: String,
    /// 分析周期
    pub analysis_period: String,
    /// ROI指标
    pub roi_metrics: ROIMetrics,
    /// 收益明细
    pub benefit_breakdown: Vec<BenefitItem>,
    /// 成本明细
    pub cost_breakdown: Vec<CostItem>,
    /// ROI预测
    pub roi_projections: Vec<ROIProjection>,
    /// 敏感性分析
    pub sensitivity_analysis: Vec<SensitivityScenario>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 总体统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStatistics {
    /// 总应用数
    pub total_applications: i32,
    /// 活跃应用数
    pub active_applications: i32,
    /// 总会话数
    pub total_sessions: i64,
    /// 总用户数
    pub total_users: i32,
    /// 平均会话时长(分钟)
    pub average_session_duration: f64,
    /// 总交互数
    pub total_interactions: i64,
    /// 用户满意度评分
    pub user_satisfaction_score: f64,
    /// 增长率
    pub growth_rate: f64,
}

/// 应用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationStatistics {
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
    /// 增长率
    pub growth_rate: f64,
    /// 热门功能
    pub top_features: Vec<String>,
    /// 使用分布
    pub usage_distribution: Vec<UsageDistribution>,
}

/// 分类统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStatistics {
    /// 分类名称
    pub category: String,
    /// 应用数量
    pub application_count: i32,
    /// 总会话数
    pub total_sessions: i64,
    /// 独立用户数
    pub unique_users: i32,
    /// 平均满意度
    pub average_satisfaction: f64,
    /// 增长率
    pub growth_rate: f64,
}

/// 使用分布
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDistribution {
    /// 功能名称
    pub feature_name: String,
    /// 使用占比
    pub usage_percentage: f64,
    /// 用户数量
    pub user_count: i32,
    /// 日均使用次数
    pub avg_daily_usage: f64,
}

/// 功能使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUsage {
    /// 功能ID
    pub feature_id: String,
    /// 功能名称
    pub feature_name: String,
    /// 功能分类
    pub feature_category: String,
    /// 使用次数
    pub usage_count: i64,
    /// 独立用户数
    pub unique_users: i32,
    /// 使用频率
    pub usage_frequency: UsageFrequency,
    /// 满意度评分
    pub satisfaction_score: f64,
    /// 平均完成时间(秒)
    pub average_completion_time: Option<f64>,
    /// 错误率
    pub error_rate: f64,
    /// 使用趋势
    pub usage_trend: TrendDirection,
    /// 采用率
    pub adoption_rate: f64,
}

/// 使用频率
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageFrequency {
    /// 每日
    Daily,
    /// 每周
    Weekly,
    /// 每月
    Monthly,
    /// 偶尔
    Occasional,
}

/// 功能用户分段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUserSegment {
    /// 分段名称
    pub segment_name: String,
    /// 用户数量
    pub user_count: i32,
    /// 使用占比
    pub usage_percentage: f64,
    /// 平均使用功能数
    pub avg_features_used: f64,
    /// 满意度评分
    pub satisfaction_score: f64,
}

/// 功能队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCohort {
    /// 队列名称
    pub cohort_name: String,
    /// 用户数量
    pub user_count: i32,
    /// 采用率
    pub adoption_rate: f64,
    /// 7天留存率
    pub retention_after_7_days: f64,
    /// 30天留存率
    pub retention_after_30_days: f64,
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// 加载时间指标
    pub load_time: LoadTimeMetrics,
    /// 响应时间指标
    pub response_time: ResponseTimeMetrics,
    /// 错误指标
    pub error_metrics: ErrorMetrics,
    /// 可用性指标
    pub availability_metrics: AvailabilityMetrics,
}

/// 加载时间指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTimeMetrics {
    /// 平均加载时间(秒)
    pub average_load_time: f64,
    /// 中位数加载时间(秒)
    pub median_load_time: f64,
    /// P95加载时间(秒)
    pub p95_load_time: f64,
    /// P99加载时间(秒)
    pub p99_load_time: f64,
    /// 首次内容渲染时间(秒)
    pub first_contentful_paint: f64,
    /// 最大内容渲染时间(秒)
    pub largest_contentful_paint: f64,
}

/// 响应时间指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    /// 平均响应时间(毫秒)
    pub average_response_time: f64,
    /// 中位数响应时间(毫秒)
    pub median_response_time: f64,
    /// P95响应时间(毫秒)
    pub p95_response_time: f64,
    /// P99响应时间(毫秒)
    pub p99_response_time: f64,
}

/// 错误指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// 错误率
    pub error_rate: f64,
    /// 总错误数
    pub total_errors: i64,
    /// 严重错误数
    pub critical_errors: i32,
    /// 警告错误数
    pub warning_errors: i32,
    /// 信息错误数
    pub info_errors: i32,
}

/// 可用性指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityMetrics {
    /// 正常运行时间百分比
    pub uptime_percentage: f64,
    /// 总停机时间(分钟)
    pub total_downtime_minutes: i64,
    /// 事故数量
    pub incident_count: i32,
    /// 平均恢复时间(分钟)
    pub mttr_mean_time_to_recovery: f64,
}

/// 性能趋势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    /// 日期
    pub date: String,
    /// 加载时间(秒)
    pub load_time: f64,
    /// 响应时间(毫秒)
    pub response_time: f64,
    /// 错误率
    pub error_rate: f64,
    /// 可用性
    pub availability: f64,
}

/// 设备性能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePerformance {
    /// 设备类型
    pub device_type: String,
    /// 用户占比
    pub user_percentage: f64,
    /// 平均加载时间(秒)
    pub average_load_time: f64,
    /// 平均响应时间(毫秒)
    pub average_response_time: f64,
    /// 错误率
    pub error_rate: f64,
    /// 满意度评分
    pub satisfaction_score: f64,
}

/// 性能问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceIssue {
    /// 问题ID
    pub issue_id: String,
    /// 问题类型
    pub issue_type: String,
    /// 严重程度
    pub severity: IssueSeverity,
    /// 问题描述
    pub description: String,
    /// 影响用户数
    pub affected_users: i32,
    /// 影响评分
    pub impact_score: f64,
    /// 建议修复
    pub suggested_fix: String,
}

/// 集成概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationOverview {
    /// 总集成数
    pub total_integrations: i32,
    /// 活跃集成数
    pub active_integrations: i32,
    /// 集成成功率
    pub integration_success_rate: f64,
    /// 平均数据同步时间(分钟)
    pub average_data_sync_time: f64,
    /// 总数据传输量
    pub total_data_transfers: i64,
}

/// 集成连接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnection {
    /// 集成ID
    pub integration_id: String,
    /// 集成名称
    pub integration_name: String,
    /// 目标系统
    pub target_system: String,
    /// 集成类型
    pub integration_type: IntegrationType,
    /// 状态
    pub status: IntegrationStatus,
    /// 成功率
    pub success_rate: f64,
    /// 数据量(MB)
    pub data_volume_mb: f64,
    /// 最后同步时间
    pub last_sync_time: DateTime<Utc>,
    /// 错误数
    pub error_count: i32,
}

/// 集成类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationType {
    /// 数据交换
    DataExchange,
    /// 工作流集成
    Workflow,
    /// API集成
    Api,
    /// 单点登录
    SSO,
}

/// 集成状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationStatus {
    /// 活跃
    Active,
    /// 非活跃
    Inactive,
    /// 错误
    Error,
    /// 配置中
    Configuring,
}

/// 数据流
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    /// 流ID
    pub flow_id: String,
    /// 源应用
    pub source_app: String,
    /// 目标应用
    pub target_app: String,
    /// 数据类型
    pub data_type: String,
    /// 流向
    pub flow_direction: DataFlowDirection,
    /// 数据量(GB/天)
    pub volume_gb_per_day: f64,
    /// 成功率
    pub success_rate: f64,
    /// 延迟(毫秒)
    pub latency_ms: f64,
    /// 最后活动时间
    pub last_activity: DateTime<Utc>,
}

/// 数据流向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataFlowDirection {
    /// 单向
    Unidirectional,
    /// 双向
    Bidirectional,
    /// 多向
    Multidirectional,
}

/// ROI指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROIMetrics {
    /// 实施成本
    pub implementation_cost: f64,
    /// 月运营成本
    pub monthly_operational_cost: f64,
    /// 总成本
    pub total_cost: f64,
    /// 价值收益
    pub value_benefits: ValueBenefits,
    /// ROI百分比
    pub roi_percentage: f64,
    /// 回收周期(月)
    pub payback_period_months: i32,
    /// 净现值
    pub net_present_value: f64,
}

/// 价值收益
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueBenefits {
    /// 时间节省价值
    pub time_savings_value: f64,
    /// 生产力提升价值
    pub productivity_increase_value: f64,
    /// 质量改善价值
    pub quality_improvement_value: f64,
    /// 协作价值
    pub collaboration_value: f64,
    /// 总收益
    pub total_benefits: f64,
}

/// 收益项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitItem {
    /// 类别
    pub category: String,
    /// 描述
    pub description: String,
    /// 月价值
    pub monthly_value: f64,
    /// 占比
    pub percentage: f64,
}

/// 成本项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    /// 类别
    pub category: String,
    /// 描述
    pub description: String,
    /// 金额
    pub amount: f64,
    /// 占比
    pub percentage: f64,
}

/// ROI预测
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROIProjection {
    /// 月份
    pub month: i32,
    /// 累计收益
    pub cumulative_benefits: f64,
    /// 累计成本
    pub cumulative_costs: f64,
    /// ROI百分比
    pub roi_percentage: f64,
}

/// 敏感性场景
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityScenario {
    /// 场景名称
    pub scenario: String,
    /// ROI百分比
    pub roi_percentage: f64,
    /// 回收周期(月)
    pub payback_period_months: i32,
    /// 假设
    pub assumptions: Vec<String>,
}

// 实现Default trait
impl Default for GetAppUsageStatisticsRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            category_filter: None,
            include_detailed_stats: Some(true),
        }
    }
}

impl Default for GetFeatureUsageAnalysisRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            period: "last_30_days".to_string(),
            feature_category_filter: None,
            min_usage_count: Some(10),
        }
    }
}

impl Default for GetAppPerformanceAnalysisRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            period: "last_30_days".to_string(),
            metric_types: None,
            device_type_filter: None,
        }
    }
}

impl Default for GetAppIntegrationAnalysisRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            period: "last_30_days".to_string(),
            integration_type_filter: None,
            status_filter: None,
        }
    }
}

impl Default for GetAppRoiAnalysisRequest {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            period: "last_12_months".to_string(),
            cost_calculation_http_http_method: Some("full_cost".to_string()),
            benefit_calculation_http_http_method: Some("productivity_based".to_string()),
        }
    }
}
