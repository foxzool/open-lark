//! Analytics数据分析服务 v1版本
//!
//! 提供企业级数据分析的完整功能：
//! - 数据概览和统计分析
//! - 用户行为分析和洞察
//! - 应用使用统计和趋势分析
//! - 自定义报表和数据可视化
//! - 实时监控和智能告警

use crate::core::config::Config;
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// Analytics服务 v1版本
#[derive(Debug, Clone)]
pub struct AnalyticsServiceV1 {
    pub config: Config,
    pub overview: OverviewService,
    pub user_analytics: UserAnalyticsService,
    pub app_analytics: AppAnalyticsService,
    pub reports: ReportService,
    pub insights: InsightsService,
    pub monitoring: MonitoringService,
}

impl AnalyticsServiceV1 {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            overview: OverviewService::new(config.clone()),
            user_analytics: UserAnalyticsService::new(config.clone()),
            app_analytics: AppAnalyticsService::new(config.clone()),
            reports: ReportService::new(config.clone()),
            insights: InsightsService::new(config.clone()),
            monitoring: MonitoringService::new(config),
        }
    }
}

// 导入所有子模块
pub mod app_analytics;
pub mod insights;
pub mod monitoring;
pub mod overview;
pub mod reports;
pub mod user_analytics;

// 重新导出服务
pub use app_analytics::AppAnalyticsService;
pub use insights::InsightsService;
pub use monitoring::MonitoringService;
pub use overview::OverviewService;
pub use reports::ReportService;
pub use user_analytics::UserAnalyticsService;

// ==================== 核心数据模型 ====================

/// 时间周期枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimePeriod {
    /// 日
    Daily,
    /// 周
    Weekly,
    /// 月
    Monthly,
    /// 季度
    Quarterly,
    /// 年
    Yearly,
    /// 自定义
    Custom(String),
}

/// 分析维度枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    /// 时间维度
    Time,
    /// 部门维度
    Department,
    /// 用户维度
    User,
    /// 应用维度
    Application,
    /// 地区维度
    Region,
    /// 设备维度
    Device,
    /// 自定义维度
    Custom(String),
}

/// 指标类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    /// 计数指标
    Count,
    /// 求和指标
    Sum,
    /// 平均值指标
    Average,
    /// 百分比指标
    Percentage,
    /// 比率指标
    Ratio,
    /// 最大值指标
    Max,
    /// 最小值指标
    Min,
    /// 中位数指标
    Median,
}

/// 数据趋势方向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    /// 上升
    Up,
    /// 下降
    Down,
    /// 持平
    Stable,
    /// 波动
    Volatile,
}

/// 数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// 时间戳
    pub timestamp: i64,
    /// 数值
    pub value: f64,
    /// 标签
    pub labels: std::collections::HashMap<String, String>,
}

/// 时间序列数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    /// 指标名称
    pub metric_name: String,
    /// 数据点列表
    pub data_points: Vec<DataPoint>,
    /// 时间周期
    pub period: TimePeriod,
    /// 单位
    pub unit: Option<String>,
}

/// 数据汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSummary {
    /// 总计
    pub total: f64,
    /// 平均值
    pub average: f64,
    /// 最小值
    pub min: f64,
    /// 最大值
    pub max: f64,
    /// 中位数
    pub median: f64,
    /// 标准差
    pub standard_deviation: Option<f64>,
    /// 数据点数量
    pub count: i64,
}

/// 对比分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// 当前值
    pub current_value: f64,
    /// 对比值
    pub comparison_value: f64,
    /// 变化值
    pub change_value: f64,
    /// 变化百分比
    pub change_percentage: f64,
    /// 趋势方向
    pub trend_direction: TrendDirection,
    /// 统计显著性
    pub statistical_significance: Option<f64>,
}

/// 分组统计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedStatistics {
    /// 分组名称
    pub group_name: String,
    /// 分组值
    pub group_value: String,
    /// 指标值
    pub metric_value: f64,
    /// 占比
    pub percentage: Option<f64>,
    /// 排名
    pub rank: Option<i32>,
    /// 子项列表
    pub sub_groups: Option<Vec<GroupedStatistics>>,
}

/// 分析查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQuery {
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 时间周期
    pub period: TimePeriod,
    /// 维度列表
    pub dimensions: Vec<Dimension>,
    /// 指标列表
    pub metrics: Vec<String>,
    /// 过滤条件
    pub filters: std::collections::HashMap<String, serde_json::Value>,
    /// 分组字段
    pub group_by: Option<Vec<String>>,
    /// 排序字段
    pub order_by: Option<Vec<String>>,
    /// 结果限制
    pub limit: Option<i32>,
    /// 分页偏移
    pub offset: Option<i32>,
}

/// 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResult {
    /// 查询参数
    pub query: AnalyticsQuery,
    /// 时间序列数据
    pub time_series: Vec<TimeSeries>,
    /// 数据汇总
    pub summary: DataSummary,
    /// 对比分析
    pub comparisons: Vec<ComparisonResult>,
    /// 分组统计
    pub grouped_stats: Vec<GroupedStatistics>,
    /// 数据点总数
    pub total_data_points: i64,
    /// 查询耗时(毫秒)
    pub query_duration_ms: i64,
    /// 缓存命中
    pub cache_hit: bool,
}

// 实现Default trait
impl Default for TimePeriod {
    fn default() -> Self {
        TimePeriod::Daily
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Time
    }
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::Count
    }
}

impl Default for AnalyticsQuery {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            start_time: now - 86400 * 30, // 默认30天前
            end_time: now,
            period: TimePeriod::Daily,
            dimensions: vec![Dimension::Time],
            metrics: vec!["count".to_string()],
            filters: std::collections::HashMap::new(),
            group_by: None,
            order_by: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

/// 报表章节类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionType {
    /// 概要章节
    Summary,
    /// 详细章节
    Detailed,
    /// 图表章节
    Chart,
    /// 表格章节
    Table,
    /// 文本章节
    Text,
}

/// 对齐方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
    /// 两端对齐
    Justify,
}
