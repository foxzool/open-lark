//!
//! 定义所有服务模块共享的核心数据类型，避免循环导入问题

use serde::{Deserialize, Serialize};
use crate::core::SDKResult;

/// 安全级别枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityLevel {
    /// 低风险
    Low,
    /// 中等风险
    Medium,
    /// 高风险
    High,
    /// 关键风险
    Critical,
}

/// 合规标准枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStandard {
    /// GDPR
    GDPR,
    /// ISO 27001
    ISO27001,
    /// SOC 2
    SOC2,
    /// HIPAA
    HIPAA,
    /// 等保2.0
    MLPS2,
    /// 个人信息保护法
    PIPL,
    /// 数据安全法
    DSL,
    /// 网络安全法
    CSL,
    /// 自定义标准
    Custom(String),
}

/// 合规状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    /// 合规
    Compliant,
    /// 部分合规
    PartiallyCompliant,
    /// 不合规
    NonCompliant,
    /// 待评估
    Pending,
    /// 不适用
    NotApplicable,
}

/// 威胁类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatType {
    /// 恶意软件
    Malware,
    /// 钓鱼攻击
    Phishing,
    /// 数据泄露
    DataBreach,
    /// 内部威胁
    InsiderThreat,
    /// DDoS攻击
    DDoS,
    /// 高级持续性威胁
    APT,
    /// 社会工程学
    SocialEngineering,
    /// 零日漏洞
    ZeroDay,
    /// 身份盗用
    IdentityTheft,
    /// 自定义威胁
    Custom(String),
}

/// 风险状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskStatus {
    /// 开放
    Open,
    /// 处理中
    InProgress,
    /// 已缓解
    Mitigated,
    /// 已接受
    Accepted,
    /// 已关闭
    Closed,
}

/// 权限级别枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    /// 无权限
    None,
    /// 只读
    Read,
    /// 读写
    ReadWrite,
    /// 管理员
    Admin,
    /// 超级管理员
    SuperAdmin,
}

/// 审计操作枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    /// 用户登录
    UserLogin,
    /// 用户登出
    UserLogout,
    /// 数据访问
    DataAccess,
    /// 数据修改
    DataModification,
    /// 权限变更
    PermissionChange,
    /// 系统配置
    SystemConfiguration,
    /// 安全事件
    SecurityEvent,
    /// 策略变更
    PolicyChange,
    /// 自定义操作
    Custom(String),
}

/// 风险可能性枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLikelihood {
    /// 极不可能
    VeryUnlikely,
    /// 不可能
    Unlikely,
    /// 可能
    Possible,
    /// 很可能
    Likely,
    /// 非常可能
    VeryLikely,
    /// 几乎确定
    AlmostCertain,
}

/// 风险影响枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskImpact {
    /// 可忽略
    Negligible,
    /// 轻微
    Minor,
    /// 中等
    Moderate,
    /// 重大
    Significant,
    /// 严重
    Severe,
    /// 灾难性
    Catastrophic,
}

/// 审计结果枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditResult {
    /// 成功
    Success,
    /// 失败
    Failure,
    /// 部分成功
    PartialSuccess,
    /// 被阻止
    Blocked,
    /// 待处理
    Pending,
    /// 错误
    Error,
}
