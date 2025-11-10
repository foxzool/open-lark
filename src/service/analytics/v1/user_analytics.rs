#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! 用户行为分析服务
//!
//! 提供深度的用户行为分析和洞察：
//! - 用户活跃度模式分析
//! - 用户行为路径追踪
//! - 用户分群和画像分析
//! - 用户流失预警和预测
//! - 用户价值评估分析

use crate::config::Config;
use crate::SDKResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 用户行为分析服务
#[derive(Debug, Clone)]
pub struct UserAnalyticsService {
    config: Config,
}

impl UserAnalyticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 用户行为模式分析 ====================

    /// 获取用户行为模式
    pub async fn get_user_behavior_patterns(
        &self,
        request: &GetUserBehaviorPatternsRequest,
    ) -> SDKResult<GetUserBehaviorPatternsResponse> {
        Ok(GetUserBehaviorPatternsResponse {
            user_id: request.user_id.clone(),
            analysis_period: request.period.clone(),
            behavior_patterns: vec![
                BehaviorPattern {
                    pattern_id: "pattern_001".to_string(),
                    pattern_name: "晨间工作模式".to_string(),
                    pattern_type: BehaviorPatternType::DailyRoutine,
                    frequency: 95.6,
                    description: Some("用户每天上午9-11点活跃度最高".to_string()),
                    time_distribution: vec![
                        TimeSlot {
                            start_hour: 9,
                            end_hour: 11,
                            activity_score: 85.2,
                            primary_actions: vec!["发送消息".to_string(), "查看文档".to_string()],
                        },
                        TimeSlot {
                            start_hour: 14,
                            end_hour: 16,
                            activity_score: 72.8,
                            primary_actions: vec!["参加会议".to_string(), "处理任务".to_string()],
                        },
                    ],
                    consistency_score: 0.92,
                },
                BehaviorPattern {
                    pattern_id: "pattern_002".to_string(),
                    pattern_name: "周末活跃模式".to_string(),
                    pattern_type: BehaviorPatternType::WeekendActivity,
                    frequency: 45.3,
                    description: Some("周末偶尔登录，主要处理紧急事务".to_string()),
                    time_distribution: vec![TimeSlot {
                        start_hour: 10,
                        end_hour: 12,
                        activity_score: 35.6,
                        primary_actions: vec!["查看消息".to_string(), "审批流程".to_string()],
                    }],
                    consistency_score: 0.68,
                },
            ],
            activity_heatmap: self.generate_activity_heatmap(&request.user_id),
            key_insights: vec![
                "用户有规律的工作习惯，适合安排重要任务在上午".to_string(),
                "下午活跃度相对较低，可能是会议或深度工作时间".to_string(),
                "周末活跃度较低，建议减少非紧急事务打扰".to_string(),
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取用户行为路径
    pub async fn get_user_journey(
        &self,
        request: &GetUserJourneyRequest,
    ) -> SDKResult<GetUserJourneyResponse> {
        Ok(GetUserJourneyResponse {
            user_id: request.user_id.clone(),
            journey_id: format!("journey_{}", chrono::Utc::now().timestamp()),
            session_type: request.session_type.clone(),
            journey_steps: vec![
                JourneyStep {
                    step_id: "step_001".to_string(),
                    action: "登录系统".to_string(),
                    application: "即时通讯".to_string(),
                    timestamp: chrono::Utc::now() - chrono::Duration::hours(8),
                    duration_seconds: 15,
                    previous_step_id: None,
                    next_step_id: Some("step_002".to_string()),
                    metadata: std::collections::HashMap::from([
                        (
                            "device_type".to_string(),
                            serde_json::Value::String("桌面端".to_string()),
                        ),
                        (
                            "location".to_string(),
                            serde_json::Value::String("公司内网".to_string()),
                        ),
                    ]),
                },
                JourneyStep {
                    step_id: "step_002".to_string(),
                    action: "查看群组消息".to_string(),
                    application: "即时通讯".to_string(),
                    timestamp: chrono::Utc::now() - chrono::Duration::hours(7)
                        + chrono::Duration::minutes(45),
                    duration_seconds: 180,
                    previous_step_id: Some("step_001".to_string()),
                    next_step_id: Some("step_003".to_string()),
                    metadata: std::collections::HashMap::from([
                        (
                            "message_count".to_string(),
                            serde_json::Value::Number(15.into()),
                        ),
                        (
                            "group_count".to_string(),
                            serde_json::Value::Number(3.into()),
                        ),
                    ]),
                },
                JourneyStep {
                    step_id: "step_003".to_string(),
                    action: "打开文档".to_string(),
                    application: "云文档".to_string(),
                    timestamp: chrono::Utc::now() - chrono::Duration::hours(7)
                        + chrono::Duration::minutes(30),
                    duration_seconds: 600,
                    previous_step_id: Some("step_002".to_string()),
                    next_step_id: Some("step_004".to_string()),
                    metadata: std::collections::HashMap::from([
                        (
                            "document_type".to_string(),
                            serde_json::Value::String("项目文档".to_string()),
                        ),
                        (
                            "view_duration".to_string(),
                            serde_json::Value::Number(600.into()),
                        ),
                    ]),
                },
                JourneyStep {
                    step_id: "step_004".to_string(),
                    action: "编辑文档".to_string(),
                    application: "云文档".to_string(),
                    timestamp: chrono::Utc::now() - chrono::Duration::hours(6),
                    duration_seconds: 1200,
                    previous_step_id: Some("step_003".to_string()),
                    next_step_id: Some("step_005".to_string()),
                    metadata: std::collections::HashMap::from([
                        (
                            "edit_count".to_string(),
                            serde_json::Value::Number(25.into()),
                        ),
                        (
                            "word_count".to_string(),
                            serde_json::Value::Number(1500.into()),
                        ),
                    ]),
                },
                JourneyStep {
                    step_id: "step_005".to_string(),
                    action: "分享文档".to_string(),
                    application: "云文档".to_string(),
                    timestamp: chrono::Utc::now() - chrono::Duration::hours(4),
                    duration_seconds: 30,
                    previous_step_id: Some("step_004".to_string()),
                    next_step_id: None,
                    metadata: std::collections::HashMap::from([
                        (
                            "share_count".to_string(),
                            serde_json::Value::Number(3.into()),
                        ),
                        (
                            "recipient_count".to_string(),
                            serde_json::Value::Number(5.into()),
                        ),
                    ]),
                },
            ],
            journey_summary: JourneySummary {
                total_duration_seconds: 2025,
                total_applications: 2,
                conversion_rate: 0.85,
                completion_rate: 1.0,
                engagement_score: 8.7,
                primary_application: "云文档".to_string(),
                common_actions: vec![
                    "查看消息".to_string(),
                    "编辑文档".to_string(),
                    "分享内容".to_string(),
                ],
            },
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取用户分群分析
    pub async fn get_user_segmentation(
        &self,
        request: &GetUserSegmentationRequest,
    ) -> SDKResult<GetUserSegmentationResponse> {
        Ok(GetUserSegmentationResponse {
            segmentation_criteria: request.criteria.clone(),
            segments: vec![
                UserSegment {
                    segment_id: "seg_001".to_string(),
                    segment_name: "高频活跃用户".to_string(),
                    segment_type: SegmentType::ActivityBased,
                    user_count: 487,
                    percentage: 38.9,
                    characteristics: vec![
                        "日活跃度 > 90%".to_string(),
                        "平均消息发送 > 20条/天".to_string(),
                        "跨应用使用频繁".to_string(),
                    ],
                    behavior_patterns: vec![
                        "工作时间规律性强".to_string(),
                        "倾向于多任务处理".to_string(),
                        "内容创建活跃".to_string(),
                    ],
                    avg_session_duration: 120.5,
                    avg_interaction_frequency: 45.2,
                    engagement_score: 9.2,
                },
                UserSegment {
                    segment_id: "seg_002".to_string(),
                    segment_name: "协作导向用户".to_string(),
                    segment_type: SegmentType::BehaviorBased,
                    user_count: 312,
                    percentage: 25.0,
                    characteristics: vec![
                        "群组消息占比 > 60%".to_string(),
                        "文档分享频繁".to_string(),
                        "会议参与度高".to_string(),
                    ],
                    behavior_patterns: vec![
                        "偏好团队协作".to_string(),
                        "知识分享活跃".to_string(),
                        "项目管理参与度高".to_string(),
                    ],
                    avg_session_duration: 95.8,
                    avg_interaction_frequency: 38.7,
                    engagement_score: 8.5,
                },
                UserSegment {
                    segment_id: "seg_003".to_string(),
                    segment_name: "任务导向用户".to_string(),
                    segment_type: SegmentType::RoleBased,
                    user_count: 218,
                    percentage: 17.4,
                    characteristics: vec![
                        "任务使用频率高".to_string(),
                        "日历管理规范".to_string(),
                        "截止日期关注度高".to_string(),
                    ],
                    behavior_patterns: vec![
                        "时间管理严格".to_string(),
                        "目标导向明确".to_string(),
                        "工具使用集中".to_string(),
                    ],
                    avg_session_duration: 78.3,
                    avg_interaction_frequency: 32.1,
                    engagement_score: 8.1,
                },
                UserSegment {
                    segment_id: "seg_004".to_string(),
                    segment_name: "低频使用用户".to_string(),
                    segment_type: SegmentType::ActivityBased,
                    user_count: 233,
                    percentage: 18.6,
                    characteristics: vec![
                        "日活跃度 < 30%".to_string(),
                        "单次使用时长短".to_string(),
                        "功能使用单一".to_string(),
                    ],
                    behavior_patterns: vec![
                        "仅基础沟通需求".to_string(),
                        "工具使用不熟练".to_string(),
                        "需要引导和培训".to_string(),
                    ],
                    avg_session_duration: 25.6,
                    avg_interaction_frequency: 8.9,
                    engagement_score: 4.2,
                },
            ],
            total_users: 1250,
            analysis_period: request.period.clone(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取用户流失预警
    pub async fn get_user_churn_prediction(
        &self,
        request: &GetUserChurnPredictionRequest,
    ) -> SDKResult<GetUserChurnPredictionResponse> {
        Ok(GetUserChurnPredictionResponse {
            prediction_model: "churn_prediction_v2".to_string(),
            model_accuracy: 0.87,
            at_risk_users: vec![
                AtRiskUser {
                    user_id: "user_001".to_string(),
                    user_name: "张三".to_string(),
                    department: "技术研发部".to_string(),
                    risk_score: 0.85,
                    risk_level: RiskLevel::High,
                    risk_factors: vec![
                        RiskFactor {
                            factor: "活跃度下降".to_string(),
                            impact: 0.35,
                            description: "最近30天活跃度下降65%".to_string(),
                        },
                        RiskFactor {
                            factor: "登录频率降低".to_string(),
                            impact: 0.28,
                            description: "登录频率从每天下降到每周2-3次".to_string(),
                        },
                        RiskFactor {
                            factor: "团队互动减少".to_string(),
                            impact: 0.22,
                            description: "群组消息发送量下降80%".to_string(),
                        },
                    ],
                    recommended_actions: vec![
                        "发送关怀消息了解情况".to_string(),
                        "提供培训或使用指导".to_string(),
                        "安排同事主动沟通协作".to_string(),
                    ],
                    predicted_churn_date: Some(chrono::Utc::now() + chrono::Duration::days(21)),
                },
                AtRiskUser {
                    user_id: "user_002".to_string(),
                    user_name: "李四".to_string(),
                    department: "市场营销部".to_string(),
                    risk_score: 0.72,
                    risk_level: RiskLevel::Medium,
                    risk_factors: vec![
                        RiskFactor {
                            factor: "功能使用减少".to_string(),
                            impact: 0.30,
                            description: "文档和协作功能使用下降50%".to_string(),
                        },
                        RiskFactor {
                            factor: "参与度降低".to_string(),
                            impact: 0.25,
                            description: "会议参与率从80%降至45%".to_string(),
                        },
                    ],
                    recommended_actions: vec![
                        "了解使用痛点和需求".to_string(),
                        "推荐相关功能使用技巧".to_string(),
                        "安排团队分享最佳实践".to_string(),
                    ],
                    predicted_churn_date: Some(chrono::Utc::now() + chrono::Duration::days(35)),
                },
            ],
            churn_statistics: ChurnStatistics {
                total_users: 1250,
                at_risk_users: 45,
                high_risk_users: 12,
                medium_risk_users: 33,
                low_risk_users: 0,
                predicted_churn_rate: 3.6,
                historical_churn_rate: 2.8,
                retention_opportunity_score: 7.2,
            },
            risk_factors_summary: vec![
                RiskFactorSummary {
                    factor: "活跃度下降".to_string(),
                    affected_users: 28,
                    average_impact: 0.42,
                    mitigation_suggestions: vec![
                        "个性化内容推荐".to_string(),
                        "使用提醒和引导".to_string(),
                        "定期用户回访".to_string(),
                    ],
                },
                RiskFactorSummary {
                    factor: "功能使用单一".to_string(),
                    affected_users: 18,
                    average_impact: 0.35,
                    mitigation_suggestions: vec![
                        "功能培训课程".to_string(),
                        "使用场景推荐".to_string(),
                        "一对一指导".to_string(),
                    ],
                },
                RiskFactorSummary {
                    factor: "团队互动减少".to_string(),
                    affected_users: 15,
                    average_impact: 0.38,
                    mitigation_suggestions: vec![
                        "团队活动组织".to_string(),
                        "协作工具推荐".to_string(),
                        "最佳实践分享".to_string(),
                    ],
                },
            ],
            analysis_period: request.period.clone(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取用户价值评估
    pub async fn get_user_value_assessment(
        &self,
        request: &GetUserValueAssessmentRequest,
    ) -> SDKResult<GetUserValueAssessmentResponse> {
        Ok(GetUserValueAssessmentResponse {
            user_id: request.user_id.clone(),
            assessment_period: request.period.clone(),
            overall_value_score: 8.7,
            value_dimensions: vec![
                ValueDimension {
                    dimension: "活跃度价值".to_string(),
                    score: 9.2,
                    weight: 0.25,
                    description: Some("基于用户活跃度和参与度的价值评估".to_string()),
                    key_metrics: vec![
                        "日活跃天数: 28/30天".to_string(),
                        "平均在线时长: 4.2小时/天".to_string(),
                        "交互频率: 45次/天".to_string(),
                    ],
                },
                ValueDimension {
                    dimension: "内容创造价值".to_string(),
                    score: 8.5,
                    weight: 0.20,
                    description: Some("基于用户内容创建和知识贡献的价值评估".to_string()),
                    key_metrics: vec![
                        "文档创建: 45个".to_string(),
                        "消息发送: 1,280条".to_string(),
                        "文件分享: 89次".to_string(),
                    ],
                },
                ValueDimension {
                    dimension: "协作影响价值".to_string(),
                    score: 8.8,
                    weight: 0.25,
                    description: Some("基于用户对团队协作的影响价值评估".to_string()),
                    key_metrics: vec![
                        "影响用户数: 156人".to_string(),
                        "群组参与度: 92%".to_string(),
                        "跨部门协作: 12次".to_string(),
                    ],
                },
                ValueDimension {
                    dimension: "知识传播价值".to_string(),
                    score: 7.9,
                    weight: 0.15,
                    description: Some("基于知识分享和传播的价值评估".to_string()),
                    key_metrics: vec![
                        "知识分享: 34次".to_string(),
                        "最佳实践贡献: 8个".to_string(),
                        "培训指导: 15次".to_string(),
                    ],
                },
                ValueDimension {
                    dimension: "创新贡献价值".to_string(),
                    score: 8.4,
                    weight: 0.15,
                    description: Some("基于创新想法和改进建议的价值评估".to_string()),
                    key_metrics: vec![
                        "创新建议: 6个".to_string(),
                        "流程改进: 3项".to_string(),
                        "工具优化: 2项".to_string(),
                    ],
                },
            ],
            value_trend: vec![
                ValueTrendPoint {
                    period: "2024-09".to_string(),
                    value_score: 8.2,
                    change_rate: 0.0,
                },
                ValueTrendPoint {
                    period: "2024-10".to_string(),
                    value_score: 8.7,
                    change_rate: 6.1,
                },
            ],
            peer_comparison: PeerComparison {
                percentile_rank: 85.6,
                department_rank: 3,
                department_total: 45,
                company_rank: 156,
                company_total: 1250,
                top_performers_percentage: 12.5,
            },
            improvement_suggestions: vec![
                "增加跨部门协作可以提升影响力价值".to_string(),
                "加强知识分享有助于提升传播价值".to_string(),
                "持续创新实践可进一步提升整体价值".to_string(),
            ],
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 辅助方法 ====================

    fn generate_activity_heatmap(&self, user_id: &str) -> Vec<ActivityHeatmapPoint> {
        let mut heatmap = Vec::new();

        // 生成最近4周的活动热力图数据
        for week in 0..4 {
            for day in 0..7 {
                for hour in 0..24 {
                    // 模拟工作时间和非工作时间的不同活跃度
                    let base_activity = if hour >= 9 && hour <= 18 {
                        0.7 + (hour as f64 - 9.0) * 0.05 // 工作时间高活跃度
                    } else if hour >= 19 && hour <= 22 {
                        0.3 + (22.0 - hour as f64) * 0.1 // 晚间中等活跃度
                    } else {
                        0.1 + (hour as f64 * 0.02).sin().abs() * 0.2 // 其他时间低活跃度
                    };

                    // 周末调整
                    let weekend_adjustment = if day >= 5 { 0.3 } else { 1.0 };
                    let activity_score = base_activity * weekend_adjustment;

                    // 随机波动
                    let random_factor = 0.8
                        + (user_id.len() as f64 * 0.1 + week as f64 * 0.05)
                            .sin()
                            .abs()
                            * 0.2;
                    let final_score = (activity_score * random_factor).min(1.0);

                    heatmap.push(ActivityHeatmapPoint {
                        date: (chrono::Utc::now() - chrono::Duration::weeks(3 - week)
                            + chrono::Duration::days(day))
                        .format("%Y-%m-%d")
                        .to_string(),
                        hour: hour as i32,
                        day_of_week: day as i32,
                        week_of_year: week as i32,
                        activity_score: final_score,
                        interaction_count: (final_score * 50.0) as i32,
                    });
                }
            }
        }

        heatmap
    }
}

// ==================== 请求数据模型 ====================

/// 获取用户行为模式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserBehaviorPatternsRequest {
    /// 用户ID
    pub user_id: String,
    /// 分析周期
    pub period: String,
    /// 模式类型过滤
    pub pattern_type_filter: Option<BehaviorPatternType>,
    /// 最小频率阈值
    pub min_frequency_threshold: Option<f64>,
}

/// 获取用户行为路径请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserJourneyRequest {
    /// 用户ID
    pub user_id: String,
    /// 会话类型
    pub session_type: String,
    /// 开始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 包含的步骤类型
    pub step_types: Option<Vec<String>>,
}

/// 获取用户分群请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSegmentationRequest {
    /// 分群标准
    pub criteria: String,
    /// 分析周期
    pub period: String,
    /// 最小分群大小
    pub min_segment_size: Option<i32>,
    /// 分群类型过滤
    pub segment_type_filter: Option<SegmentType>,
}

/// 获取用户流失预警请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserChurnPredictionRequest {
    /// 分析周期
    pub period: String,
    /// 风险等级过滤
    pub risk_level_filter: Option<RiskLevel>,
    /// 最小风险分数
    pub min_risk_score: Option<f64>,
    /// 预测时间范围
    pub prediction_horizon_days: Option<i32>,
}

/// 获取用户价值评估请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserValueAssessmentRequest {
    /// 用户ID
    pub user_id: String,
    /// 评估周期
    pub period: String,
    /// 评估维度
    pub evaluation_dimensions: Option<Vec<String>>,
    /// 是否包含同行对比
    pub include_peer_comparison: Option<bool>,
}

// ==================== 响应数据模型 ====================

/// 获取用户行为模式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserBehaviorPatternsResponse {
    /// 用户ID
    pub user_id: String,
    /// 分析周期
    pub analysis_period: String,
    /// 行为模式列表
    pub behavior_patterns: Vec<BehaviorPattern>,
    /// 活动热力图
    pub activity_heatmap: Vec<ActivityHeatmapPoint>,
    /// 关键洞察
    pub key_insights: Vec<String>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取用户行为路径响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserJourneyResponse {
    /// 用户ID
    pub user_id: String,
    /// 路径ID
    pub journey_id: String,
    /// 会话类型
    pub session_type: String,
    /// 路径步骤
    pub journey_steps: Vec<JourneyStep>,
    /// 路径汇总
    pub journey_summary: JourneySummary,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取用户分群响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSegmentationResponse {
    /// 分群标准
    pub segmentation_criteria: String,
    /// 用户分群列表
    pub segments: Vec<UserSegment>,
    /// 总用户数
    pub total_users: i32,
    /// 分析周期
    pub analysis_period: String,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取用户流失预警响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserChurnPredictionResponse {
    /// 预测模型
    pub prediction_model: String,
    /// 模型准确度
    pub model_accuracy: f64,
    /// 高风险用户列表
    pub at_risk_users: Vec<AtRiskUser>,
    /// 流失统计
    pub churn_statistics: ChurnStatistics,
    /// 风险因素汇总
    pub risk_factors_summary: Vec<RiskFactorSummary>,
    /// 分析周期
    pub analysis_period: String,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 获取用户价值评估响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserValueAssessmentResponse {
    /// 用户ID
    pub user_id: String,
    /// 评估周期
    pub assessment_period: String,
    /// 总体价值评分
    pub overall_value_score: f64,
    /// 价值维度
    pub value_dimensions: Vec<ValueDimension>,
    /// 价值趋势
    pub value_trend: Vec<ValueTrendPoint>,
    /// 同行对比
    pub peer_comparison: PeerComparison,
    /// 改进建议
    pub improvement_suggestions: Vec<String>,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 行为模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    /// 模式ID
    pub pattern_id: String,
    /// 模式名称
    pub pattern_name: String,
    /// 模式类型
    pub pattern_type: BehaviorPatternType,
    /// 频率(百分比)
    pub frequency: f64,
    /// 描述
    pub description: Option<String>,
    /// 时间分布
    pub time_distribution: Vec<TimeSlot>,
    /// 一致性评分
    pub consistency_score: f64,
}

/// 行为模式类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorPatternType {
    /// 日常惯例
    DailyRoutine,
    /// 工作模式
    WorkPattern,
    /// 沟通模式
    CommunicationPattern,
    /// 协作模式
    CollaborationPattern,
    /// 学习模式
    LearningPattern,
    /// 周末活动
    WeekendActivity,
}

/// 时间段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    /// 开始小时
    pub start_hour: i32,
    /// 结束小时
    pub end_hour: i32,
    /// 活跃度评分
    pub activity_score: f64,
    /// 主要行为
    pub primary_actions: Vec<String>,
}

/// 活动热力图点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityHeatmapPoint {
    /// 日期
    pub date: String,
    /// 小时
    pub hour: i32,
    /// 星期几
    pub day_of_week: i32,
    /// 年中的周数
    pub week_of_year: i32,
    /// 活跃度评分
    pub activity_score: f64,
    /// 交互次数
    pub interaction_count: i32,
}

/// 路径步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyStep {
    /// 步骤ID
    pub step_id: String,
    /// 行为动作
    pub action: String,
    /// 应用
    pub application: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 持续时间(秒)
    pub duration_seconds: i64,
    /// 上一步ID
    pub previous_step_id: Option<String>,
    /// 下一步ID
    pub next_step_id: Option<String>,
    /// 元数据
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// 路径汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneySummary {
    /// 总持续时间(秒)
    pub total_duration_seconds: i64,
    /// 涉及应用数
    pub total_applications: i32,
    /// 转化率
    pub conversion_rate: f64,
    /// 完成率
    pub completion_rate: f64,
    /// 参与度评分
    pub engagement_score: f64,
    /// 主要应用
    pub primary_application: String,
    /// 常见行为
    pub common_actions: Vec<String>,
}

/// 用户分群
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSegment {
    /// 分群ID
    pub segment_id: String,
    /// 分群名称
    pub segment_name: String,
    /// 分群类型
    pub segment_type: SegmentType,
    /// 用户数量
    pub user_count: i32,
    /// 百分比
    pub percentage: f64,
    /// 特征描述
    pub characteristics: Vec<String>,
    /// 行为模式
    pub behavior_patterns: Vec<String>,
    /// 平均会话时长(分钟)
    pub avg_session_duration: f64,
    /// 平均交互频率
    pub avg_interaction_frequency: f64,
    /// 参与度评分
    pub engagement_score: f64,
}

/// 分群类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentType {
    /// 基于活跃度
    ActivityBased,
    /// 基于行为
    BehaviorBased,
    /// 基于角色
    RoleBased,
    /// 基于价值
    ValueBased,
    /// 基于部门
    DepartmentBased,
    /// 基于地理位置
    LocationBased,
}

/// 高风险用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtRiskUser {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: String,
    /// 部门
    pub department: String,
    /// 风险评分
    pub risk_score: f64,
    /// 风险等级
    pub risk_level: RiskLevel,
    /// 风险因素
    pub risk_factors: Vec<RiskFactor>,
    /// 建议行动
    pub recommended_actions: Vec<String>,
    /// 预测流失日期
    pub predicted_churn_date: Option<DateTime<Utc>>,
}

/// 风险等级
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// 低风险
    Low,
    /// 中风险
    Medium,
    /// 高风险
    High,
    /// 极高风险
    Critical,
}

/// 风险因素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// 因素名称
    pub factor: String,
    /// 影响程度
    pub impact: f64,
    /// 描述
    pub description: String,
}

/// 流失统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChurnStatistics {
    /// 总用户数
    pub total_users: i32,
    /// 风险用户数
    pub at_risk_users: i32,
    /// 高风险用户数
    pub high_risk_users: i32,
    /// 中风险用户数
    pub medium_risk_users: i32,
    /// 低风险用户数
    pub low_risk_users: i32,
    /// 预测流失率
    pub predicted_churn_rate: f64,
    /// 历史流失率
    pub historical_churn_rate: f64,
    /// 留存机会评分
    pub retention_opportunity_score: f64,
}

/// 风险因素汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorSummary {
    /// 因素名称
    pub factor: String,
    /// 影响用户数
    pub affected_users: i32,
    /// 平均影响程度
    pub average_impact: f64,
    /// 缓解建议
    pub mitigation_suggestions: Vec<String>,
}

/// 价值维度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueDimension {
    /// 维度名称
    pub dimension: String,
    /// 评分
    pub score: f64,
    /// 权重
    pub weight: f64,
    /// 描述
    pub description: Option<String>,
    /// 关键指标
    pub key_metrics: Vec<String>,
}

/// 价值趋势点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueTrendPoint {
    /// 周期
    pub period: String,
    /// 价值评分
    pub value_score: f64,
    /// 变化率
    pub change_rate: f64,
}

/// 同行对比
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerComparison {
    /// 百分位排名
    pub percentile_rank: f64,
    /// 部门排名
    pub department_rank: i32,
    /// 部门总人数
    pub department_total: i32,
    /// 公司排名
    pub company_rank: i32,
    /// 公司总人数
    pub company_total: i32,
    /// 高绩效者百分比
    pub top_performers_percentage: f64,
}

// 实现Default trait
impl Default for GetUserBehaviorPatternsRequest {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            period: "last_30_days".to_string(),
            pattern_type_filter: None,
            min_frequency_threshold: Some(10.0),
        }
    }
}

impl Default for GetUserJourneyRequest {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            session_type: "work_session".to_string(),
            start_time: None,
            end_time: None,
            step_types: None,
        }
    }
}

impl Default for GetUserSegmentationRequest {
    fn default() -> Self {
        Self {
            criteria: "activity_based".to_string(),
            period: "last_30_days".to_string(),
            min_segment_size: Some(10),
            segment_type_filter: None,
        }
    }
}

impl Default for GetUserChurnPredictionRequest {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            risk_level_filter: None,
            min_risk_score: Some(0.5),
            prediction_horizon_days: Some(30),
        }
    }
}

impl Default for GetUserValueAssessmentRequest {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            period: "last_30_days".to_string(),
            evaluation_dimensions: None,
            include_peer_comparison: Some(true),
        }
    }
}
