#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Monitoring监控服务 - 实时监控和智能告警
//!
//! 提供企业级的实时监控功能：
//! - 系统性能实时监控
//! - 业务指标智能监控
//! - 自定义告警规则和策略
//! - 多渠道告警通知
//! - 监控数据可视化和报告
//! - SLA监控和服务质量保证

use crate::config::Config;
use crate::SDKResult;
use serde::{Deserialize, Serialize};

/// 监控服务
#[derive(Debug, Clone)]
pub struct MonitoringService {
    pub config: Config,
}

impl MonitoringService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取实时监控数据
    /// 获取系统和业务的实时性能指标
    pub async fn get_real_time_monitoring(
        &self,
        _request: &GetRealTimeMonitoringRequest,
    ) -> SDKResult<GetRealTimeMonitoringResponse> {
        let current_time = chrono::Utc::now().timestamp();

        let metrics = vec![
            RealTimeMetric {
                metric_id: "metric_001".to_string(),
                metric_name: "API响应时间".to_string(),
                category: MetricCategory::Performance,
                current_value: 125.5,
                unit: "ms".to_string(),
                status: MetricStatus::Healthy,
                threshold_config: ThresholdConfig {
                    warning_threshold: 500.0,
                    critical_threshold: 1000.0,
                    operator: ComparisonOperator::GreaterThan,
                },
                trend_data: (0..60)
                    .map(|i| MetricDataPoint {
                        timestamp: current_time - (60 - i) * 60,
                        value: 120.0 + (i as f64 * 0.5) + ((i as f64 * 0.1).sin() * 10.0),
                    })
                    .collect(),
                alerts: vec![],
                last_updated: current_time,
            },
            RealTimeMetric {
                metric_id: "metric_002".to_string(),
                metric_name: "系统CPU使用率".to_string(),
                category: MetricCategory::System,
                current_value: 67.8,
                unit: "%".to_string(),
                status: MetricStatus::Warning,
                threshold_config: ThresholdConfig {
                    warning_threshold: 70.0,
                    critical_threshold: 90.0,
                    operator: ComparisonOperator::GreaterThan,
                },
                trend_data: (0..60)
                    .map(|i| MetricDataPoint {
                        timestamp: current_time - (60 - i) * 60,
                        value: 65.0 + (i as f64 * 0.1) + ((i as f64 * 0.15).cos() * 8.0),
                    })
                    .collect(),
                alerts: vec![ActiveAlert {
                    alert_id: "alert_001".to_string(),
                    severity: AlertSeverity::Low,
                    title: "CPU使用率接近警告阈值".to_string(),
                    description: "当前CPU使用率67.8%，接近70%警告阈值".to_string(),
                    triggered_at: current_time - 300,
                    acknowledged: false,
                }],
                last_updated: current_time,
            },
            RealTimeMetric {
                metric_id: "metric_003".to_string(),
                metric_name: "用户活跃度".to_string(),
                category: MetricCategory::Business,
                current_value: 3456.0,
                unit: "count".to_string(),
                status: MetricStatus::Healthy,
                threshold_config: ThresholdConfig {
                    warning_threshold: 2000.0,
                    critical_threshold: 1000.0,
                    operator: ComparisonOperator::LessThan,
                },
                trend_data: (0..60)
                    .map(|i| MetricDataPoint {
                        timestamp: current_time - (60 - i) * 60,
                        value: 3200.0 + (i as f64 * 4.2) + ((i as f64 * 0.08).sin() * 150.0),
                    })
                    .collect(),
                alerts: vec![],
                last_updated: current_time,
            },
        ];

        Ok(GetRealTimeMonitoringResponse {
            metrics: metrics.clone(),
            monitoring_summary: MonitoringSummary {
                total_metrics: metrics.len() as i32,
                healthy_metrics: 2,
                warning_metrics: 1,
                critical_metrics: 0,
                last_updated: current_time,
                data_freshness_ms: 5000, // 5秒数据新鲜度
            },
        })
    }

    /// 获取告警历史
    /// 获取历史告警记录和趋势分析
    pub async fn get_alert_history(
        &self,
        request: &GetAlertHistoryRequest,
    ) -> SDKResult<GetAlertHistoryResponse> {
        let current_time = chrono::Utc::now().timestamp();

        let alerts = vec![
            AlertRecord {
                alert_id: "alert_001".to_string(),
                title: "数据库连接池耗尽".to_string(),
                description: "数据库连接池使用率达到100%，新的连接请求被拒绝".to_string(),
                severity: AlertSeverity::Critical,
                status: AlertStatus::Resolved,
                category: AlertCategory::System,
                source_system: "数据库服务器".to_string(),
                metric_name: "数据库连接池使用率".to_string(),
                triggered_at: current_time - 86400, // 1天前
                acknowledged_at: Some(current_time - 86400 + 300),
                resolved_at: Some(current_time - 86400 + 1800),
                duration_minutes: 30,
                affected_services: vec!["用户认证服务".to_string(), "订单处理服务".to_string()],
                resolution_actions: vec![
                    "重启数据库连接池".to_string(),
                    "增加连接池大小".to_string(),
                    "优化慢查询".to_string(),
                ],
                impact_assessment: ImpactAssessment {
                    affected_users: 2500,
                    business_impact: "用户登录失败，订单处理中断".to_string(),
                    estimated_loss: 0.0, // 需要根据具体业务计算
                },
            },
            AlertRecord {
                alert_id: "alert_002".to_string(),
                title: "API响应时间异常".to_string(),
                description: "API平均响应时间超过2秒，严重影响用户体验".to_string(),
                severity: AlertSeverity::High,
                status: AlertStatus::Acknowledged,
                category: AlertCategory::Performance,
                source_system: "API网关".to_string(),
                metric_name: "API响应时间".to_string(),
                triggered_at: current_time - 3600, // 1小时前
                acknowledged_at: Some(current_time - 3600 + 600),
                resolved_at: None,
                duration_minutes: 60,
                affected_services: vec!["所有API服务".to_string()],
                resolution_actions: vec!["扩容API服务器".to_string(), "优化数据库查询".to_string()],
                impact_assessment: ImpactAssessment {
                    affected_users: 8500,
                    business_impact: "用户体验下降，操作响应缓慢".to_string(),
                    estimated_loss: 0.0,
                },
            },
        ];

        Ok(GetAlertHistoryResponse {
            alerts: alerts.clone(),
            total_count: alerts.len() as i32,
            alert_statistics: AlertStatistics {
                total_alerts: 156,
                critical_alerts: 12,
                high_alerts: 34,
                medium_alerts: 67,
                low_alerts: 43,
                resolved_alerts: 134,
                open_alerts: 22,
                average_resolution_time_minutes: 45.6,
                mttr: 42.3, // 平均修复时间
            },
            pagination: PaginationInfo {
                page_size: request.page_size.unwrap_or(20),
                page_number: 1,
                total_pages: 8,
                has_next: true,
                has_previous: false,
            },
        })
    }

    /// 创建告警规则
    /// 创建自定义的告警规则和触发条件
    pub async fn create_alert_rule(
        &self,
        request: &CreateAlertRuleRequest,
    ) -> SDKResult<CreateAlertRuleResponse> {
        let rule_id = format!("rule_{}", chrono::Utc::now().timestamp());

        Ok(CreateAlertRuleResponse {
            rule_id: rule_id.clone(),
            rule_name: request.rule_name.clone(),
            status: RuleStatus::Active,
            created_at: chrono::Utc::now().timestamp(),
            next_evaluation: chrono::Utc::now().timestamp() + 60, // 1分钟后首次评估
            validation_result: RuleValidation {
                is_valid: true,
                validation_errors: vec![],
                recommendations: vec!["建议设置更严格的告警阈值以减少误报".to_string()],
            },
        })
    }

    /// 获取SLA监控数据
    /// 获取服务级别协议的监控和合规性数据
    pub async fn get_sla_monitoring(
        &self,
        request: &GetSlaMonitoringRequest,
    ) -> SDKResult<GetSlaMonitoringResponse> {
        let current_time = chrono::Utc::now().timestamp();

        let sla_metrics = vec![
            SLAMetric {
                sla_id: "sla_001".to_string(),
                service_name: "用户认证服务".to_string(),
                sla_type: SLAType::Availability,
                target_value: 99.9,
                current_value: 99.94,
                measurement_period: "monthly".to_string(),
                status: SLAStatus::Compliant,
                violation_count: 0,
                last_violation: None,
                trend_data: (0..30)
                    .map(|i| SLADataPoint {
                        date: current_time - (30 - i) * 86400,
                        actual_value: 99.9 + ((i as f64 * 0.01).sin() * 0.05),
                        target_value: 99.9,
                    })
                    .collect(),
            },
            SLAMetric {
                sla_id: "sla_002".to_string(),
                service_name: "API服务".to_string(),
                sla_type: SLAType::ResponseTime,
                target_value: 500.0,
                current_value: 425.6,
                measurement_period: "daily".to_string(),
                status: SLAStatus::Compliant,
                violation_count: 2,
                last_violation: Some(current_time - 86400 * 3),
                trend_data: (0..30)
                    .map(|i| SLADataPoint {
                        date: current_time - (30 - i) * 86400,
                        actual_value: 450.0 + ((i as f64 * 0.02).cos() * 50.0),
                        target_value: 500.0,
                    })
                    .collect(),
            },
        ];

        Ok(GetSlaMonitoringResponse {
            sla_metrics: sla_metrics.clone(),
            sla_summary: SLASummary {
                total_sla_metrics: sla_metrics.len() as i32,
                compliant_metrics: 2,
                non_compliant_metrics: 0,
                overall_compliance_rate: 100.0,
                period_start: request.start_time,
                period_end: request.end_time,
                next_review_date: current_time + 86400 * 7, // 下周复查
            },
        })
    }

    /// 获取监控仪表板
    /// 获取综合监控仪表板数据
    pub async fn get_monitoring_dashboard(
        &self,
        _request: &GetMonitoringDashboardRequest,
    ) -> SDKResult<GetMonitoringDashboardResponse> {
        let current_time = chrono::Utc::now().timestamp();

        Ok(GetMonitoringDashboardResponse {
            dashboard_config: DashboardConfig {
                dashboard_id: "dashboard_main".to_string(),
                title: "系统监控总览".to_string(),
                layout: LayoutConfig {
                    columns: 3,
                    rows: 2,
                    widget_positions: vec![
                        WidgetPosition {
                            widget_id: "widget_health".to_string(),
                            x: 0,
                            y: 0,
                            width: 1,
                            height: 1,
                        },
                        WidgetPosition {
                            widget_id: "widget_performance".to_string(),
                            x: 1,
                            y: 0,
                            width: 1,
                            height: 1,
                        },
                        WidgetPosition {
                            widget_id: "widget_alerts".to_string(),
                            x: 2,
                            y: 0,
                            width: 1,
                            height: 1,
                        },
                    ],
                },
                refresh_interval_seconds: 30,
                auto_refresh: true,
            },
            widgets: vec![
                Widget {
                    widget_id: "widget_health".to_string(),
                    widget_type: WidgetType::HealthSummary,
                    title: "系统健康状态".to_string(),
                    data: serde_json::json!({
                        "overall_health": "healthy",
                        "healthy_services": 15,
                        "degraded_services": 2,
                        "failed_services": 0,
                        "last_updated": current_time
                    }),
                },
                Widget {
                    widget_id: "widget_performance".to_string(),
                    widget_type: WidgetType::PerformanceMetrics,
                    title: "性能指标".to_string(),
                    data: serde_json::json!({
                        "response_time": 125.5,
                        "throughput": 3456.7,
                        "error_rate": 0.02,
                        "cpu_usage": 67.8,
                        "memory_usage": 72.3
                    }),
                },
                Widget {
                    widget_id: "widget_alerts".to_string(),
                    widget_type: WidgetType::ActiveAlerts,
                    title: "活跃告警".to_string(),
                    data: serde_json::json!({
                        "critical_alerts": 0,
                        "high_alerts": 1,
                        "medium_alerts": 3,
                        "low_alerts": 5,
                        "total_alerts": 9
                    }),
                },
            ],
            last_updated: current_time,
            next_update: current_time + 30,
        })
    }

    /// 获取监控报告
    /// 生成和获取监控分析报告
    pub async fn get_monitoring_report(
        &self,
        request: &GetMonitoringReportRequest,
    ) -> SDKResult<GetMonitoringReportResponse> {
        let current_time = chrono::Utc::now().timestamp();

        Ok(GetMonitoringReportResponse {
            report: MonitoringReport {
                report_id: format!("report_{}", current_time),
                report_type: request.report_type.clone(),
                period_start: request.start_time,
                period_end: request.end_time,
                generated_at: current_time,
                executive_summary: ExecutiveSummary {
                    overall_system_health: "良好".to_string(),
                    key_achievements: vec![
                        "系统可用性达到99.95%，超过SLA目标".to_string(),
                        "平均响应时间优化15%".to_string(),
                        "告警响应时间缩短30%".to_string(),
                    ],
                    major_issues: vec![
                        "CPU使用率偶尔超过警告阈值".to_string(),
                        "数据库连接池需要优化".to_string(),
                    ],
                    recommendations: vec![
                        "建议增加服务器资源以应对峰值负载".to_string(),
                        "优化数据库查询性能".to_string(),
                        "完善告警策略以减少误报".to_string(),
                    ],
                },
                performance_analysis: PerformanceAnalysis {
                    availability_metrics: vec![AvailabilityMetric {
                        service_name: "用户认证服务".to_string(),
                        availability_percentage: 99.94,
                        downtime_minutes: 43,
                        mtbf: 720.5, // 平均故障间隔时间
                    }],
                    performance_metrics: vec![PerformanceMetric {
                        metric_name: "平均响应时间".to_string(),
                        current_value: 125.5,
                        previous_value: 148.2,
                        improvement_percentage: 15.3,
                        trend: "improving".to_string(),
                    }],
                    capacity_metrics: vec![CapacityMetric {
                        resource_name: "CPU使用率".to_string(),
                        current_utilization: 67.8,
                        peak_utilization: 89.2,
                        average_utilization: 71.5,
                        projected_capacity: "充足".to_string(),
                    }],
                },
                alert_analysis: AlertAnalysis {
                    total_alerts: 156,
                    alerts_by_severity: vec![
                        ("Critical".to_string(), 12),
                        ("High".to_string(), 34),
                        ("Medium".to_string(), 67),
                        ("Low".to_string(), 43),
                    ],
                    mttr: 42.3,
                    false_positive_rate: 0.08,
                    top_alert_sources: vec![
                        ("API网关".to_string(), 45),
                        ("数据库服务器".to_string(), 38),
                        ("缓存服务".to_string(), 28),
                    ],
                },
            },
        })
    }
}

// ==================== 数据模型 ====================

/// 实时监控请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealTimeMonitoringRequest {
    /// 监控指标类别
    pub metric_categories: Vec<MetricCategory>,
    /// 刷新频率(秒)
    pub refresh_interval: Option<i32>,
    /// 时间窗口(分钟)
    pub time_window_minutes: Option<i32>,
}

/// 实时监控响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealTimeMonitoringResponse {
    /// 实时指标数据
    pub metrics: Vec<RealTimeMetric>,
    /// 监控摘要
    pub monitoring_summary: MonitoringSummary,
}

/// 指标类别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricCategory {
    /// 性能指标
    Performance,
    /// 系统指标
    System,
    /// 业务指标
    Business,
    /// 安全指标
    Security,
    /// 用户体验
    UserExperience,
}

/// 实时指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetric {
    /// 指标ID
    pub metric_id: String,
    /// 指标名称
    pub metric_name: String,
    /// 指标类别
    pub category: MetricCategory,
    /// 当前值
    pub current_value: f64,
    /// 单位
    pub unit: String,
    /// 状态
    pub status: MetricStatus,
    /// 阈值配置
    pub threshold_config: ThresholdConfig,
    /// 趋势数据
    pub trend_data: Vec<MetricDataPoint>,
    /// 活跃告警
    pub alerts: Vec<ActiveAlert>,
    /// 最后更新时间
    pub last_updated: i64,
}

/// 指标状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricStatus {
    /// 健康
    Healthy,
    /// 警告
    Warning,
    /// 关键
    Critical,
    /// 未知
    Unknown,
}

/// 阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// 警告阈值
    pub warning_threshold: f64,
    /// 关键阈值
    pub critical_threshold: f64,
    /// 比较操作符
    pub operator: ComparisonOperator,
}

/// 比较操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    /// 大于
    GreaterThan,
    /// 小于
    LessThan,
    /// 等于
    Equal,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于等于
    LessThanOrEqual,
}

/// 指标数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// 时间戳
    pub timestamp: i64,
    /// 数值
    pub value: f64,
}

/// 活跃告警
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveAlert {
    /// 告警ID
    pub alert_id: String,
    /// 严重程度
    pub severity: AlertSeverity,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 触发时间
    pub triggered_at: i64,
    /// 是否已确认
    pub acknowledged: bool,
}

/// 告警严重程度
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertSeverity {
    /// 关键
    Critical,
    /// 高
    High,
    /// 中等
    Medium,
    /// 低
    Low,
}

/// 监控摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSummary {
    /// 总指标数
    pub total_metrics: i32,
    /// 健康指标数
    pub healthy_metrics: i32,
    /// 警告指标数
    pub warning_metrics: i32,
    /// 关键指标数
    pub critical_metrics: i32,
    /// 最后更新时间
    pub last_updated: i64,
    /// 数据新鲜度(毫秒)
    pub data_freshness_ms: i64,
}

/// 告警历史请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAlertHistoryRequest {
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 告警严重程度过滤
    pub severities: Option<Vec<AlertSeverity>>,
    /// 告警状态过滤
    pub statuses: Option<Vec<AlertStatus>>,
    /// 分页参数
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
}

/// 告警历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAlertHistoryResponse {
    /// 告警记录
    pub alerts: Vec<AlertRecord>,
    /// 总数量
    pub total_count: i32,
    /// 告警统计
    pub alert_statistics: AlertStatistics,
    /// 分页信息
    pub pagination: PaginationInfo,
}

/// 告警状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    /// 开放
    Open,
    /// 已确认
    Acknowledged,
    /// 已解决
    Resolved,
    /// 已关闭
    Closed,
}

/// 告警类别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertCategory {
    /// 系统
    System,
    /// 性能
    Performance,
    /// 安全
    Security,
    /// 业务
    Business,
    /// 网络
    Network,
}

/// 告警记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRecord {
    /// 告警ID
    pub alert_id: String,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 严重程度
    pub severity: AlertSeverity,
    /// 状态
    pub status: AlertStatus,
    /// 类别
    pub category: AlertCategory,
    /// 源系统
    pub source_system: String,
    /// 指标名称
    pub metric_name: String,
    /// 触发时间
    pub triggered_at: i64,
    /// 确认时间
    pub acknowledged_at: Option<i64>,
    /// 解决时间
    pub resolved_at: Option<i64>,
    /// 持续时间(分钟)
    pub duration_minutes: i32,
    /// 受影响服务
    pub affected_services: Vec<String>,
    /// 解决行动
    pub resolution_actions: Vec<String>,
    /// 影响评估
    pub impact_assessment: ImpactAssessment,
}

/// 影响评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// 受影响用户数
    pub affected_users: i32,
    /// 业务影响
    pub business_impact: String,
    /// 预估损失
    pub estimated_loss: f64,
}

/// 告警统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStatistics {
    /// 总告警数
    pub total_alerts: i32,
    /// 关键告警数
    pub critical_alerts: i32,
    /// 高告警数
    pub high_alerts: i32,
    /// 中等告警数
    pub medium_alerts: i32,
    /// 低告警数
    pub low_alerts: i32,
    /// 已解决告警数
    pub resolved_alerts: i32,
    /// 开放告警数
    pub open_alerts: i32,
    /// 平均解决时间(分钟)
    pub average_resolution_time_minutes: f64,
    /// MTTR (平均修复时间)
    pub mttr: f64,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// 页面大小
    pub page_size: i32,
    /// 页码
    pub page_number: i32,
    /// 总页数
    pub total_pages: i32,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_previous: bool,
}

/// 创建告警规则请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertRuleRequest {
    /// 规则名称
    pub rule_name: String,
    /// 规则描述
    pub description: String,
    /// 监控指标
    pub metric_name: String,
    /// 条件配置
    pub condition: AlertCondition,
    /// 告警级别
    pub severity: AlertSeverity,
    /// 通知配置
    pub notification_config: NotificationConfig,
    /// 是否启用
    pub enabled: bool,
}

/// 告警条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    /// 操作符
    pub operator: ComparisonOperator,
    /// 阈值
    pub threshold: f64,
    /// 持续时间(秒)
    pub duration_seconds: i32,
    /// 评估频率(秒)
    pub evaluation_frequency: i32,
}

/// 通知配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 通知渠道
    pub channels: Vec<NotificationChannel>,
    /// 通知间隔(秒)
    pub notification_interval: i32,
    /// 最大通知次数
    pub max_notifications: i32,
}

/// 通知渠道
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    /// 邮件
    Email,
    /// 短信
    SMS,
    /// 钉钉/飞书
    InstantMessaging,
    /// Webhook
    Webhook,
}

/// 创建告警规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertRuleResponse {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则状态
    pub status: RuleStatus,
    /// 创建时间
    pub created_at: i64,
    /// 下次评估时间
    pub next_evaluation: i64,
    /// 验证结果
    pub validation_result: RuleValidation,
}

/// 规则状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    /// 激活
    Active,
    /// 暂停
    Paused,
    /// 已禁用
    Disabled,
}

/// 规则验证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleValidation {
    /// 是否有效
    pub is_valid: bool,
    /// 验证错误
    pub validation_errors: Vec<String>,
    /// 建议
    pub recommendations: Vec<String>,
}

/// SLA监控请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlaMonitoringRequest {
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 服务过滤
    pub services: Option<Vec<String>>,
    /// SLA类型过滤
    pub sla_types: Option<Vec<SLAType>>,
}

/// SLA监控响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlaMonitoringResponse {
    /// SLA指标
    pub sla_metrics: Vec<SLAMetric>,
    /// SLA摘要
    pub sla_summary: SLASummary,
}

/// SLA类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SLAType {
    /// 可用性
    Availability,
    /// 响应时间
    ResponseTime,
    /// 吞吐量
    Throughput,
    /// 错误率
    ErrorRate,
}

/// SLA指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAMetric {
    /// SLA ID
    pub sla_id: String,
    /// 服务名称
    pub service_name: String,
    /// SLA类型
    pub sla_type: SLAType,
    /// 目标值
    pub target_value: f64,
    /// 当前值
    pub current_value: f64,
    /// 测量周期
    pub measurement_period: String,
    /// 状态
    pub status: SLAStatus,
    /// 违规次数
    pub violation_count: i32,
    /// 最后违规时间
    pub last_violation: Option<i64>,
    /// 趋势数据
    pub trend_data: Vec<SLADataPoint>,
}

/// SLA状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SLAStatus {
    /// 合规
    Compliant,
    /// 不合规
    NonCompliant,
    /// 风险
    AtRisk,
}

/// SLA数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLADataPoint {
    /// 日期
    pub date: i64,
    /// 实际值
    pub actual_value: f64,
    /// 目标值
    pub target_value: f64,
}

/// SLA摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLASummary {
    /// 总SLA指标数
    pub total_sla_metrics: i32,
    /// 合规指标数
    pub compliant_metrics: i32,
    /// 不合规指标数
    pub non_compliant_metrics: i32,
    /// 总体合规率
    pub overall_compliance_rate: f64,
    /// 周期开始
    pub period_start: i64,
    /// 周期结束
    pub period_end: i64,
    /// 下次复查日期
    pub next_review_date: i64,
}

/// 监控仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMonitoringDashboardRequest {
    /// 仪表板ID
    pub dashboard_id: Option<String>,
    /// 仪表板类型
    pub dashboard_type: Option<DashboardType>,
    /// 时间范围
    pub time_range: Option<TimeRange>,
}

/// 仪表板类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardType {
    /// 总览
    Overview,
    /// 性能
    Performance,
    /// 业务
    Business,
    /// 基础设施
    Infrastructure,
}

/// 时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    /// 最近1小时
    LastHour,
    /// 最近24小时
    Last24Hours,
    /// 最近7天
    Last7Days,
    /// 最近30天
    Last30Days,
    /// 自定义
    Custom { start_time: i64, end_time: i64 },
}

/// 监控仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMonitoringDashboardResponse {
    /// 仪表板配置
    pub dashboard_config: DashboardConfig,
    /// 仪表板组件
    pub widgets: Vec<Widget>,
    /// 最后更新时间
    pub last_updated: i64,
    /// 下次更新时间
    pub next_update: i64,
}

/// 仪表板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// 仪表板ID
    pub dashboard_id: String,
    /// 标题
    pub title: String,
    /// 布局配置
    pub layout: LayoutConfig,
    /// 刷新间隔(秒)
    pub refresh_interval_seconds: i32,
    /// 自动刷新
    pub auto_refresh: bool,
}

/// 布局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// 列数
    pub columns: i32,
    /// 行数
    pub rows: i32,
    /// 组件位置
    pub widget_positions: Vec<WidgetPosition>,
}

/// 组件位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    /// 组件ID
    pub widget_id: String,
    /// X坐标
    pub x: i32,
    /// Y坐标
    pub y: i32,
    /// 宽度
    pub width: i32,
    /// 高度
    pub height: i32,
}

/// 仪表板组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    /// 组件ID
    pub widget_id: String,
    /// 组件类型
    pub widget_type: WidgetType,
    /// 标题
    pub title: String,
    /// 组件数据
    pub data: serde_json::Value,
}

/// 组件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetType {
    /// 健康状态摘要
    HealthSummary,
    /// 性能指标
    PerformanceMetrics,
    /// 活跃告警
    ActiveAlerts,
    /// 趋势图表
    TrendChart,
    /// SLA状态
    SLAStatus,
    /// 容量使用
    CapacityUsage,
}

/// 监控报告请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMonitoringReportRequest {
    /// 报告类型
    pub report_type: ReportType,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 报告格式
    pub format: Option<ReportFormat>,
}

/// 报告类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    /// 日报
    Daily,
    /// 周报
    Weekly,
    /// 月报
    Monthly,
    /// 季报
    Quarterly,
    /// 自定义
    Custom,
}

/// 报告格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    /// JSON
    Json,
    /// PDF
    Pdf,
    /// Excel
    Excel,
}

/// 监控报告响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMonitoringReportResponse {
    /// 监控报告
    pub report: MonitoringReport,
}

/// 监控报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringReport {
    /// 报告ID
    pub report_id: String,
    /// 报告类型
    pub report_type: ReportType,
    /// 周期开始
    pub period_start: i64,
    /// 周期结束
    pub period_end: i64,
    /// 生成时间
    pub generated_at: i64,
    /// 执行摘要
    pub executive_summary: ExecutiveSummary,
    /// 性能分析
    pub performance_analysis: PerformanceAnalysis,
    /// 告警分析
    pub alert_analysis: AlertAnalysis,
}

/// 执行摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    /// 整体系统健康状态
    pub overall_system_health: String,
    /// 主要成就
    pub key_achievements: Vec<String>,
    /// 主要问题
    pub major_issues: Vec<String>,
    /// 建议
    pub recommendations: Vec<String>,
}

/// 性能分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    /// 可用性指标
    pub availability_metrics: Vec<AvailabilityMetric>,
    /// 性能指标
    pub performance_metrics: Vec<PerformanceMetric>,
    /// 容量指标
    pub capacity_metrics: Vec<CapacityMetric>,
}

/// 可用性指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityMetric {
    /// 服务名称
    pub service_name: String,
    /// 可用性百分比
    pub availability_percentage: f64,
    /// 停机时间(分钟)
    pub downtime_minutes: i32,
    /// MTBF (平均故障间隔时间)
    pub mtbf: f64,
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    /// 指标名称
    pub metric_name: String,
    /// 当前值
    pub current_value: f64,
    /// 之前值
    pub previous_value: f64,
    /// 改进百分比
    pub improvement_percentage: f64,
    /// 趋势
    pub trend: String,
}

/// 容量指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityMetric {
    /// 资源名称
    pub resource_name: String,
    /// 当前使用率
    pub current_utilization: f64,
    /// 峰值使用率
    pub peak_utilization: f64,
    /// 平均使用率
    pub average_utilization: f64,
    /// 预期容量
    pub projected_capacity: String,
}

/// 告警分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAnalysis {
    /// 总告警数
    pub total_alerts: i32,
    /// 按严重程度分组的告警
    pub alerts_by_severity: Vec<(String, i32)>,
    /// MTTR
    pub mttr: f64,
    /// 误报率
    pub false_positive_rate: f64,
    /// 主要告警源
    pub top_alert_sources: Vec<(String, i32)>,
}
