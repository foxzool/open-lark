//! zero_trust 服务端点常量定义
//!
//! 零信任安全架构相关 API 端点常量，包括：
//! - 身份验证增强
//! - 设备信任评估
//! - 微分段访问控制
//! - 持续监控

/// 设备信任评估
pub const DEVICE_TRUST_ASSESSMENT: &str = "/open-apis/zero_trust/v1/device/trust/assess";

/// 用户身份增强验证
pub const ENHANCED_IDENTITY_VERIFICATION: &str =
    "/open-apis/zero_trust/v1/identity/enhanced_verify";

/// 持续身份验证
pub const CONTINUOUS_IDENTITY_VERIFICATION: &str =
    "/open-apis/zero_trust/v1/identity/continuous_verify";

/// 微分段访问控制
pub const MICRO_SEGMENTATION_ACCESS: &str = "/open-apis/zero_trust/v1/access/micro_segment";

/// 上下文感知访问策略
pub const CONTEXT_AWARE_ACCESS_POLICY: &str = "/open-apis/zero_trust/v1/policy/context_aware";

/// 自适应访问控制
pub const ADAPTIVE_ACCESS_CONTROL: &str = "/open-apis/zero_trust/v1/access/adaptive";

/// 行为基线分析
pub const BEHAVIORAL_BASELINE_ANALYSIS: &str = "/open-apis/zero_trust/v1/behavior/baseline";

/// 异常行为检测
pub const ANOMALY_BEHAVIOR_DETECTION: &str = "/open-apis/zero_trust/v1/behavior/anomaly";

/// 高级审计和合规API
/// 实时安全审计
pub const REAL_TIME_SECURITY_AUDIT: &str = "/open-apis/zero_trust/v1/audit/realtime";

/// 合规状态检查
pub const COMPLIANCE_STATUS_CHECK: &str = "/open-apis/zero_trust/v1/compliance/status";

/// GDPR合规检查
pub const GDPR_COMPLIANCE_CHECK: &str = "/open-apis/zero_trust/v1/compliance/gdpr";

/// SOC2合规检查
pub const SOC2_COMPLIANCE_CHECK: &str = "/open-apis/zero_trust/v1/compliance/soc2";

/// 高级加密和密钥管理
/// 端到端加密管理
pub const END_TO_END_ENCRYPTION: &str = "/open-apis/zero_trust/v1/encryption/e2e";

/// 密钥轮换管理
pub const KEY_ROTATION_MANAGEMENT: &str = "/open-apis/zero_trust/v1/key/rotation";

/// 零知识证明
pub const ZERO_KNOWLEDGE_PROOF: &str = "/open-apis/zero_trust/v1/cryptography/zkp";

/// 威胁情报和响应
/// 威胁情报订阅
pub const THREAT_INTELLIGENCE_SUBSCRIPTION: &str = "/open-apis/zero_trust/v1/threat/intelligence";

/// 自动化威胁响应
pub const AUTOMATED_THREAT_RESPONSE: &str = "/open-apis/zero_trust/v1/threat/response";

/// 攻击链分析
pub const ATTACK_CHAIN_ANALYSIS: &str = "/open-apis/zero_trust/v1/threat/attack_chain";

/// 安全事件关联分析
pub const SECURITY_INCIDENT_CORRELATION: &str = "/open-apis/zero_trust/v1/incident/correlation";

/// 零信任网络访问
/// 零信任网络网关
pub const ZERO_TRUST_NETWORK_GATEWAY: &str = "/open-apis/zero_trust/v1/network/gateway";

/// 安全访问隧道
pub const SECURE_ACCESS_TUNNEL: &str = "/open-apis/zero_trust/v1/network/tunnel";

/// 高级权限管理
/// 细粒度权限控制
pub const FINE_GRAINED_PERMISSION_CONTROL: &str =
    "/open-apis/zero_trust/v1/permission/fine_grained";

/// 动态权限调整
pub const DYNAMIC_PERMISSION_ADJUSTMENT: &str = "/open-apis/zero_trust/v1/permission/dynamic";

/// 权限审计追踪
pub const PERMISSION_AUDIT_TRACKING: &str = "/open-apis/zero_trust/v1/permission/audit";
