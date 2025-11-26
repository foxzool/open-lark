//! Security & Compliance (安全合规管理) 数据模型

use serde::{Deserialize, Serialize};

/// 设备记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecord {
    /// 设备记录ID
    pub device_record_id: String,
    /// 设备名称
    pub device_name: String,
    /// 设备类型
    pub device_type: String,
    /// 设备品牌
    pub device_brand: Option<String>,
    /// 设备型号
    pub device_model: Option<String>,
    /// 操作系统
    pub os_type: Option<String>,
    /// 操作系统版本
    pub os_version: Option<String>,
    /// 设备序列号
    pub serial_number: Option<String>,
    /// MAC地址
    pub mac_address: Option<String>,
    /// 设备状态
    pub status: DeviceRecordStatus,
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: String,
    /// 用户部门
    pub department_name: Option<String>,
    /// 是否为个人设备
    pub personal_device: bool,
    /// 合规检查状态
    pub compliance_status: ComplianceStatus,
    /// 最后检查时间
    pub last_check_time: Option<crate::models::Timestamp>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 设备记录状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceRecordStatus {
    /// 待审核
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired,
    /// 已撤销
    Revoked,
    /// 不合规
    NonCompliant,
}

/// 合规状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    /// 合规
    Compliant,
    /// 不合规
    NonCompliant,
    /// 待检查
    Pending,
    /// 未知
    Unknown,
}

/// 设备记录创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecordRequest {
    /// 设备名称
    pub device_name: String,
    /// 设备类型
    pub device_type: String,
    /// 设备品牌
    pub device_brand: Option<String>,
    /// 设备型号
    pub device_model: Option<String>,
    /// 操作系统
    pub os_type: Option<String>,
    /// 操作系统版本
    pub os_version: Option<String>,
    /// 设备序列号
    pub serial_number: Option<String>,
    /// MAC地址
    pub mac_address: Option<String>,
    /// 是否为个人设备
    pub personal_device: Option<bool>,
    /// 扩展信息
    pub extension: Option<crate::models::ExtensionMap>,
}

/// 设备记录更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecordUpdateRequest {
    /// 设备名称
    pub device_name: Option<String>,
    /// 设备品牌
    pub device_brand: Option<String>,
    /// 设备型号
    pub device_model: Option<String>,
    /// 操作系统
    pub os_type: Option<String>,
    /// 操作系统版本
    pub os_version: Option<String>,
    /// 设备序列号
    pub serial_number: Option<String>,
    /// MAC地址
    pub mac_address: Option<String>,
    /// 合规检查状态
    pub compliance_status: Option<ComplianceStatus>,
    /// 扩展信息
    pub extension: Option<crate::models::ExtensionMap>,
}

/// 设备申报记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceApplyRecord {
    /// 申报记录ID
    pub device_apply_record_id: String,
    /// 设备记录ID
    pub device_record_id: Option<String>,
    /// 设备信息
    pub device_info: Option<DeviceRecordRequest>,
    /// 申请人ID
    pub applicant_id: String,
    /// 申请人姓名
    pub applicant_name: String,
    /// 申报原因
    pub apply_reason: String,
    /// 申报状态
    pub apply_status: ApplyStatus,
    /// 审批人ID
    pub approver_id: Option<String>,
    /// 审批人姓名
    pub approver_name: Option<String>,
    /// 审批意见
    pub approve_comment: Option<String>,
    /// 审批时间
    pub approve_time: Option<crate::models::Timestamp>,
    /// 申报时间
    pub apply_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 申报状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApplyStatus {
    /// 待审批
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
    /// 已撤销
    Revoked,
}

/// 设备申报审批请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceApplyRecordApproveRequest {
    /// 是否批准
    pub approved: bool,
    /// 审批意见
    pub comment: Option<String>,
    /// 审批备注
    pub remark: Option<String>,
}

/// OpenAPI 审计日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiLog {
    /// 日志ID
    pub log_id: String,
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 应用ID
    pub app_id: String,
    /// API路径
    pub api_path: String,
    /// 请求方法
    pub method: String,
    /// 请求状态码
    pub status_code: i32,
    /// 请求耗时（毫秒）
    pub cost_time: Option<i32>,
    /// 客户端IP
    pub client_ip: Option<String>,
    /// User-Agent
    pub user_agent: Option<String>,
    /// 请求时间
    pub request_time: crate::models::Timestamp,
    /// 响应时间
    pub response_time: crate::models::Timestamp,
    /// 错误信息
    pub error_msg: Option<String>,
    /// 请求参数
    pub request_params: Option<serde_json::Value>,
    /// 响应数据摘要
    pub response_summary: Option<String>,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// OpenAPI 日志查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiLogQueryRequest {
    /// 开始时间
    pub start_time: Option<crate::models::Timestamp>,
    /// 结束时间
    pub end_time: Option<crate::models::Timestamp>,
    /// 用户ID过滤
    pub user_id_filter: Option<String>,
    /// API路径过滤
    pub api_path_filter: Option<String>,
    /// 状态码过滤
    pub status_code_filter: Option<i32>,
    /// 应用ID过滤
    pub app_id_filter: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_direction: Option<String>,
}

/// 设备合规检查规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComplianceRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则描述
    pub rule_description: Option<String>,
    /// 规则类型
    pub rule_type: ComplianceRuleType,
    /// 规则内容
    pub rule_content: serde_json::Value,
    /// 规则状态
    pub status: crate::models::Status,
    /// 优先级
    pub priority: i32,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 合规规则类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceRuleType {
    /// 设备类型检查
    DeviceTypeCheck,
    /// 操作系统检查
    OsCheck,
    /// 安全软件检查
    SecuritySoftwareCheck,
    /// 加密检查
    EncryptionCheck,
    /// 网络访问检查
    NetworkAccessCheck,
    /// 自定义规则
    Custom,
}

/// 合规检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// 检查结果ID
    pub result_id: String,
    /// 设备记录ID
    pub device_record_id: String,
    /// 规则ID
    pub rule_id: String,
    /// 检查结果
    pub result: ComplianceResult,
    /// 检查时间
    pub check_time: crate::models::Timestamp,
    /// 检查消息
    pub message: Option<String>,
    /// 检查详情
    pub details: Option<serde_json::Value>,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 合规结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceResult {
    /// 通过
    Pass,
    /// 失败
    Fail,
    /// 警告
    Warning,
    /// 跳过
    Skip,
}
