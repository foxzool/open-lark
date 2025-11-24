//! 风险评估数据模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 风险级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// 低风险
    #[serde(rename = "low")]
    Low,
    /// 中风险
    #[serde(rename = "medium")]
    Medium,
    /// 高风险
    #[serde(rename = "high")]
    High,
    /// 严重风险
    #[serde(rename = "critical")]
    Critical,
}

/// 风险类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskType {
    /// 数据泄露
    #[serde(rename = "data_leak")]
    DataLeak,
    /// 未授权访问
    #[serde(rename = "unauthorized_access")]
    UnauthorizedAccess,
    /// 恶意软件
    #[serde(rename = "malware")]
    Malware,
    /// 网络攻击
    #[serde(rename = "network_attack")]
    NetworkAttack,
    /// 内部威胁
    #[serde(rename = "insider_threat")]
    InsiderThreat,
    /// 合规违规
    #[serde(rename = "compliance_violation")]
    ComplianceViolation,
    /// 配置错误
    #[serde(rename = "misconfiguration")]
    Misconfiguration,
    /// 弱密码
    #[serde(rename = "weak_password")]
    WeakPassword,
}

/// 风险状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskStatus {
    /// 开放
    #[serde(rename = "open")]
    Open,
    /// 处理中
    #[serde(rename = "in_progress")]
    InProgress,
    /// 已缓解
    #[serde(rename = "mitigated")]
    Mitigated,
    /// 已关闭
    #[serde(rename = "closed")]
    Closed,
    /// 误报
    #[serde(rename = "false_positive")]
    FalsePositive,
}

/// 风险评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// 风险ID
    pub risk_id: String,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 风险类型
    pub risk_type: RiskType,
    /// 风险级别
    pub risk_level: RiskLevel,
    /// 状态
    pub status: RiskStatus,
    /// 影响的资源
    pub affected_resources: Vec<String>,
    /// 检测时间
    pub detected_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 检测源
    pub detection_source: String,
    /// 置信度
    pub confidence: f32,
    /// 影响评估
    pub impact_assessment: ImpactAssessment,
    /// 缓解措施
    pub mitigation_measures: Vec<MitigationMeasure>,
    /// 相关事件
    pub related_events: Vec<String>,
    /// 标签
    pub tags: Vec<String>,
}

/// 影响评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// 机密性影响
    pub confidentiality_impact: ImpactLevel,
    /// 完整性影响
    pub integrity_impact: ImpactLevel,
    /// 可用性影响
    pub availability_impact: ImpactLevel,
    /// 业务影响
    pub business_impact: BusinessImpact,
    /// 影响用户数
    pub affected_users: u32,
    /// 潜在损失
    pub potential_loss: Option<f64>,
}

/// 影响级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImpactLevel {
    /// 无影响
    #[serde(rename = "none")]
    None,
    /// 低影响
    #[serde(rename = "low")]
    Low,
    /// 中影响
    #[serde(rename = "medium")]
    Medium,
    /// 高影响
    #[serde(rename = "high")]
    High,
    /// 严重影响
    #[serde(rename = "critical")]
    Critical,
}

/// 业务影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpact {
    /// 财务影响
    pub financial_impact: String,
    /// 运营影响
    pub operational_impact: String,
    /// 声誉影响
    pub reputational_impact: String,
    /// 合规影响
    pub compliance_impact: String,
}

/// 缓解措施
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationMeasure {
    /// 措施ID
    pub measure_id: String,
    /// 措施名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 优先级
    pub priority: MitigationPriority,
    /// 状态
    pub status: MitigationStatus,
    /// 负责人
    pub assignee: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 截止时间
    pub due_date: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 进度百分比
    pub progress_percentage: u32,
}

/// 缓解优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MitigationPriority {
    /// 低优先级
    #[serde(rename = "low")]
    Low,
    /// 中优先级
    #[serde(rename = "medium")]
    Medium,
    /// 高优先级
    #[serde(rename = "high")]
    High,
    /// 紧急优先级
    #[serde(rename = "urgent")]
    Urgent,
}

/// 缓解状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MitigationStatus {
    /// 待开始
    #[serde(rename = "pending")]
    Pending,
    /// 进行中
    #[serde(rename = "in_progress")]
    InProgress,
    /// 已完成
    #[serde(rename = "completed")]
    Completed,
    /// 已阻塞
    #[serde(rename = "blocked")]
    Blocked,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// 风险评估请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentRequest {
    /// 资源ID列表
    pub resource_ids: Vec<String>,
    /// 风险类型列表
    pub risk_types: Option<Vec<RiskType>>,
    /// 评估深度
    pub assessment_depth: Option<AssessmentDepth>,
    /// 包含建议
    pub include_recommendations: Option<bool>,
}

/// 评估深度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentDepth {
    /// 快速评估
    #[serde(rename = "quick")]
    Quick,
    /// 标准评估
    #[serde(rename = "standard")]
    Standard,
    /// 深度评估
    #[serde(rename = "deep")]
    Deep,
}

/// 风险评估响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentResponse {
    /// 评估ID
    pub assessment_id: String,
    /// 评估时间
    pub assessed_at: DateTime<Utc>,
    /// 风险列表
    pub risks: Vec<RiskAssessment>,
    /// 风险统计
    pub risk_statistics: RiskStatistics,
    /// 总体风险级别
    pub overall_risk_level: RiskLevel,
    /// 关键发现
    pub key_findings: Vec<String>,
    /// 建议措施
    pub recommendations: Vec<RiskRecommendation>,
}

/// 风险统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskStatistics {
    /// 总风险数
    pub total_risks: u32,
    /// 按级别统计
    pub risks_by_level: Vec<RiskByLevel>,
    /// 按类型统计
    pub risks_by_type: Vec<RiskByType>,
    /// 按状态统计
    pub risks_by_status: Vec<RiskByStatus>,
}

/// 按级别统计的风险
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskByLevel {
    /// 风险级别
    pub level: RiskLevel,
    /// 数量
    pub count: u32,
    /// 百分比
    pub percentage: f32,
}

/// 按类型统计的风险
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskByType {
    /// 风险类型
    pub risk_type: RiskType,
    /// 数量
    pub count: u32,
    /// 百分比
    pub percentage: f32,
}

/// 按状态统计的风险
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskByStatus {
    /// 风险状态
    pub status: RiskStatus,
    /// 数量
    pub count: u32,
    /// 百分比
    pub percentage: f32,
}

/// 风险建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRecommendation {
    /// 建议ID
    pub recommendation_id: String,
    /// 风险ID
    pub risk_id: String,
    /// 建议类型
    pub recommendation_type: RecommendationType,
    /// 优先级
    pub priority: MitigationPriority,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 实施步骤
    pub implementation_steps: Vec<String>,
    /// 预计效果
    pub expected_outcome: String,
    /// 相关资源
    pub related_resources: Vec<String>,
}

/// 建议类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationType {
    /// 预防措施
    #[serde(rename = "preventive")]
    Preventive,
    /// 纠正措施
    #[serde(rename = "corrective")]
    Corrective,
    /// 检测措施
    #[serde(rename = "detective")]
    Detective,
    /// 补偿措施
    #[serde(rename = "compensating")]
    Compensating,
}

/// 风险查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskQueryRequest {
    /// 开始时间
    pub start_time: Option<DateTime<Utc>>,
    /// 结束时间
    pub end_time: Option<DateTime<Utc>>,
    /// 风险级别列表
    pub risk_levels: Option<Vec<RiskLevel>>,
    /// 风险类型列表
    pub risk_types: Option<Vec<RiskType>>,
    /// 风险状态列表
    pub risk_statuses: Option<Vec<RiskStatus>>,
    /// 资源ID列表
    pub resource_ids: Option<Vec<String>>,
    /// 分页
    pub page: Option<u32>,
    /// 页面大小
    pub page_size: Option<u32>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
}

/// 风险查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskQueryResponse {
    /// 风险列表
    pub risks: Vec<RiskAssessment>,
    /// 总数量
    pub total_count: u32,
    /// 当前页
    pub current_page: u32,
    /// 页面大小
    pub page_size: u32,
    /// 总页数
    pub total_pages: u32,
}
