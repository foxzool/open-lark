//! 打卡流水相关模型
//!
//! 包含导入、查询、获取、删除打卡流水等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// ============================================================================
// 导入打卡流水相关模型 (batch_create)
// ============================================================================

/// 导入打卡流水请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateUserFlowRequestBody {
    /// 打卡流水列表
    pub flow_records: Vec<UserFlowRecord>,
}

/// 打卡流水记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserFlowRecord {
    /// 用户 ID
    pub user_id: String,
    /// 打卡时间，格式为 yyyy-MM-dd HH:mm:ss
    pub punch_time: String,
    /// 打卡类型
    /// - 1: 上班打卡
    /// - 2: 下班打卡
    /// - 3: 外出打卡
    pub punch_type: i32,
    /// 打卡方式
    /// - 1: 手机打卡
    /// - 2: 考勤机打卡
    /// - 3: 手动导入
    pub punch_method: i32,
    /// 打卡地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_place_name: Option<String>,
    /// 打卡地点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_place_id: Option<String>,
    /// 经度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// 纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 打卡 Wi-Fi 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_name: Option<String>,
    /// 打卡 Wi-Fi MAC 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
    /// 打卡设备 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 打卡设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 打卡照片列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_list: Option<Vec<String>>,
    /// 外勤打卡地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_address: Option<String>,
    /// 外勤打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_remark: Option<String>,
}

/// 导入打卡流水响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateUserFlowResponse {
    /// 导入结果列表
    pub results: Vec<UserFlowResult>,
}

/// 打卡流水导入结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserFlowResult {
    /// 用户 ID
    pub user_id: String,
    /// 打卡时间
    pub punch_time: String,
    /// 是否成功
    pub success: bool,
    /// 错误码（失败时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// 错误信息（失败时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

// ============================================================================
// 查询打卡流水相关模型 (query)
// ============================================================================

/// 查询打卡流水请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserFlowRequestBody {
    /// 查询的起始日期，格式为 yyyy-MM-dd
    pub start_date: String,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    pub end_date: String,
    /// 查询的用户 ID 列表，最多支持 50 个用户
    pub user_ids: Vec<String>,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 查询的打卡类型，可选值：
    /// - 1: 上班打卡
    /// - 2: 下班打卡
    /// - 3: 外出打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_type: Option<i32>,
}

/// 打卡流水信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserFlowInfo {
    /// 打卡流水 ID
    pub user_flow_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 打卡日期，格式为 yyyy-MM-dd
    pub punch_date: String,
    /// 打卡时间，格式为 yyyy-MM-dd HH:mm:ss
    pub punch_time: String,
    /// 打卡类型
    /// - 1: 上班打卡
    /// - 2: 下班打卡
    /// - 3: 外出打卡
    pub punch_type: i32,
    /// 打卡方式
    /// - 1: 手机打卡
    /// - 2: 考勤机打卡
    /// - 3: 手动导入
    pub punch_method: i32,
    /// 打卡地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_place_name: Option<String>,
    /// 打卡地点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_place_id: Option<String>,
    /// 经度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// 纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 打卡 Wi-Fi 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_name: Option<String>,
    /// 打卡 Wi-Fi MAC 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
    /// 打卡设备 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 打卡设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 打卡照片列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_list: Option<Vec<String>>,
    /// 外勤打卡地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_address: Option<String>,
    /// 外勤打卡备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_remark: Option<String>,
}

/// 查询打卡流水响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryUserFlowResponse {
    /// 打卡流水列表
    pub flow_list: Vec<UserFlowInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// ============================================================================
// 获取打卡流水相关模型 (get)
// ============================================================================

/// 获取打卡流水请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserFlowRequestBody {
    /// 打卡流水 ID
    pub user_flow_id: String,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 获取打卡流水响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUserFlowResponse {
    /// 打卡流水信息
    pub flow_info: UserFlowInfo,
}

// ============================================================================
// 删除打卡流水相关模型 (batch_del)
// ============================================================================

/// 删除打卡流水请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDelUserFlowRequestBody {
    /// 要删除的打卡流水 ID 列表
    pub user_flow_ids: Vec<String>,
}

/// 删除打卡流水结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDelUserFlowResult {
    /// 打卡流水 ID
    pub user_flow_id: String,
    /// 是否删除成功
    pub success: bool,
    /// 错误码（失败时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// 错误信息（失败时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

/// 删除打卡流水响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDelUserFlowResponse {
    /// 删除结果列表
    pub results: Vec<BatchDelUserFlowResult>,
}
