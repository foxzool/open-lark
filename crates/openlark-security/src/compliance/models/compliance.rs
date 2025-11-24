//! 合规检查数据模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 合规检查类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceType {
    /// 数据保护
    #[serde(rename = "data_protection")]
    DataProtection,
    /// 访问控制
    #[serde(rename = "access_control")]
    AccessControl,
    /// 加密合规
    #[serde(rename = "encryption")]
    Encryption,
    /// 审计日志
    #[serde(rename = "audit_logging")]
    AuditLogging,
    /// 密码策略
    #[serde(rename = "password_policy")]
    PasswordPolicy,
    /// 会话管理
    #[serde(rename = "session_management")]
    SessionManagement,
    /// 网络安全
    #[serde(rename = "network_security")]
    NetworkSecurity,
    /// 身份认证
    #[serde(rename = "identity_authentication")]
    IdentityAuthentication,
}

/// 合规状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    /// 合规
    #[serde(rename = "compliant")]
    Compliant,
    /// 不合规
    #[serde(rename = "non_compliant")]
    NonCompliant,
    /// 部分合规
    #[serde(rename = "partially_compliant")]
    PartiallyCompliant,
    /// 未知
    #[serde(rename = "unknown")]
    Unknown,
}

/// 合规检查项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    /// 检查ID
    pub check_id: String,
    /// 检查名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 合规类型
    pub compliance_type: ComplianceType,
    /// 状态
    pub status: ComplianceStatus,
    /// 严重程度
    pub severity: ComplianceSeverity,
    /// 检查时间
    pub checked_at: DateTime<Utc>,
    /// 下次检查时间
    pub next_check_at: Option<DateTime<Utc>>,
    /// 检查结果
    pub result: Option<ComplianceResult>,
    /// 建议措施
    pub recommendations: Vec<String>,
    /// 影响的资源
    pub affected_resources: Vec<String>,
}

/// 合规严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceSeverity {
    /// 低
    #[serde(rename = "low")]
    Low,
    /// 中
    #[serde(rename = "medium")]
    Medium,
    /// 高
    #[serde(rename = "high")]
    High,
    /// 严重
    #[serde(rename = "critical")]
    Critical,
}

/// 合规检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    /// 通过
    pub passed: bool,
    /// 得分
    pub score: f32,
    /// 总分
    pub total_score: f32,
    /// 检查详情
    pub details: Vec<ComplianceDetail>,
    /// 失败原因
    pub failure_reasons: Vec<String>,
}

/// 合规检查详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDetail {
    /// 检查项
    pub item: String,
    /// 期望值
    pub expected: String,
    /// 实际值
    pub actual: String,
    /// 是否通过
    pub passed: bool,
    /// 权重
    pub weight: f32,
}

/// 合规检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckRequest {
    /// 检查类型列表
    pub compliance_types: Vec<ComplianceType>,
    /// 资源ID列表
    pub resource_ids: Option<Vec<String>>,
    /// 强制重新检查
    pub force_recheck: Option<bool>,
    /// 包含详情
    pub include_details: Option<bool>,
}

/// 合规检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResponse {
    /// 检查ID
    pub check_id: String,
    /// 检查时间
    pub checked_at: DateTime<Utc>,
    /// 检查结果列表
    pub checks: Vec<ComplianceCheck>,
    /// 总体状态
    pub overall_status: ComplianceStatus,
    /// 总体得分
    pub overall_score: f32,
    /// 严重问题数量
    pub critical_issues: u32,
    /// 高级问题数量
    pub high_issues: u32,
    /// 中级问题数量
    pub medium_issues: u32,
    /// 低级问题数量
    pub low_issues: u32,
}

/// 合规报告请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportRequest {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 合规类型列表
    pub compliance_types: Option<Vec<ComplianceType>>,
    /// 包含图表数据
    pub include_charts: Option<bool>,
    /// 报告格式
    pub report_format: Option<ReportFormat>,
}

/// 报告格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    /// JSON格式
    #[serde(rename = "json")]
    Json,
    /// PDF格式
    #[serde(rename = "pdf")]
    Pdf,
    /// Excel格式
    #[serde(rename = "excel")]
    Excel,
}

/// 合规报告响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportResponse {
    /// 报告ID
    pub report_id: String,
    /// 生成时间
    pub generated_at: DateTime<Utc>,
    /// 报告周期
    pub period: ReportPeriod,
    /// 总体合规性
    pub overall_compliance: f32,
    /// 合规趋势
    pub compliance_trend: Vec<ComplianceTrendPoint>,
    /// 问题统计
    pub issue_statistics: IssueStatistics,
    /// 合规检查摘要
    pub check_summary: Vec<ComplianceCheckSummary>,
    /// 建议措施
    pub recommendations: Vec<ComplianceRecommendation>,
    /// 报告文件URL（如果有）
    pub report_url: Option<String>,
}

/// 报告周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPeriod {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
}

/// 合规趋势点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceTrendPoint {
    /// 时间点
    pub timestamp: DateTime<Utc>,
    /// 合规率
    pub compliance_rate: f32,
    /// 检查数量
    pub check_count: u32,
}

/// 问题统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatistics {
    /// 总问题数
    pub total_issues: u32,
    /// 已解决问题数
    pub resolved_issues: u32,
    /// 待解决问题数
    pub pending_issues: u32,
    /// 按严重程度统计
    pub issues_by_severity: Vec<IssueBySeverity>,
}

/// 按严重程度统计的问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueBySeverity {
    /// 严重程度
    pub severity: ComplianceSeverity,
    /// 数量
    pub count: u32,
    /// 百分比
    pub percentage: f32,
}

/// 合规检查摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckSummary {
    /// 合规类型
    pub compliance_type: ComplianceType,
    /// 检查数量
    pub check_count: u32,
    /// 通过数量
    pub passed_count: u32,
    /// 失败数量
    pub failed_count: u32,
    /// 合规率
    pub compliance_rate: f32,
}

/// 合规建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRecommendation {
    /// 建议ID
    pub recommendation_id: String,
    /// 优先级
    pub priority: RecommendationPriority,
    /// 类别
    pub category: String,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 影响的资源
    pub affected_resources: Vec<String>,
    /// 预计工作量
    pub estimated_effort: Option<String>,
    /// 截止时间
    pub due_date: Option<DateTime<Utc>>,
}

/// 建议优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationPriority {
    /// 低
    #[serde(rename = "low")]
    Low,
    /// 中
    #[serde(rename = "medium")]
    Medium,
    /// 高
    #[serde(rename = "high")]
    High,
    /// 紧急
    #[serde(rename = "urgent")]
    Urgent,
}
