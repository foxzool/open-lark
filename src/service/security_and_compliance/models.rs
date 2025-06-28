use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// OpenAPI 审计日志请求参数
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenapiLogListRequest {
    /// 页码
    pub page_token: Option<String>,
    /// 页面大小，范围：[1, 1000]
    pub page_size: Option<i32>,
    /// 开始时间（Unix 时间戳，精确到毫秒）
    pub start_time: Option<i64>,
    /// 结束时间（Unix 时间戳，精确到毫秒）  
    pub end_time: Option<i64>,
    /// 应用 ID 列表，多个用逗号分隔
    pub app_ids: Option<String>,
    /// OpenAPI 接口名列表，多个用逗号分隔
    pub apis: Option<String>,
    /// 返回码列表，多个用逗号分隔
    pub response_codes: Option<String>,
}

/// OpenAPI 审计日志项
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenapiLogItem {
    /// 日志时间（Unix 时间戳，精确到毫秒）
    pub timestamp: i64,
    /// 应用 ID
    pub app_id: String,
    /// 应用名称
    pub app_name: String,
    /// OpenAPI 接口名
    pub api: String,
    /// HTTP 方法
    pub method: String,
    /// 请求 ID
    pub request_id: String,
    /// 返回码
    pub response_code: i32,
    /// 响应时间（毫秒）
    pub response_time: i64,
    /// 用户 ID
    pub user_id: Option<String>,
    /// 租户 key
    pub tenant_key: Option<String>,
    /// IP 地址
    pub ip: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
}

/// OpenAPI 审计日志响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenapiLogListResponse {
    /// 是否还有更多页面
    pub has_more: bool,
    /// 下一页的页面令牌
    pub page_token: Option<String>,
    /// 日志项列表
    pub items: Vec<OpenapiLogItem>,
}

/// 行为审计日志请求参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogGetRequest {
    /// 获取数据类型，固定值 "all"
    pub data_type: String,
    /// 开始时间（Unix 时间戳，精确到毫秒）
    pub start_time: i64,
    /// 结束时间（Unix 时间戳，精确到毫秒）
    pub end_time: i64,
    /// 页码，最小值为 1
    pub page: Option<i32>,
    /// 页面大小，范围：[1, 1000]
    pub page_size: Option<i32>,
    /// 审计日志类型列表
    pub audit_types: Option<Vec<String>>,
    /// 操作人 ID 列表
    pub operator_ids: Option<Vec<String>>,
    /// 被操作对象 ID 列表
    pub object_ids: Option<Vec<String>>,
}

/// 行为审计日志项
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogItem {
    /// 日志 ID
    pub log_id: String,
    /// 日志时间（Unix 时间戳，精确到毫秒）
    pub timestamp: i64,
    /// 审计日志类型
    pub audit_type: String,
    /// 操作人 ID
    pub operator_id: String,
    /// 操作人姓名
    pub operator_name: String,
    /// 被操作对象 ID
    pub object_id: Option<String>,
    /// 被操作对象名称
    pub object_name: Option<String>,
    /// 操作详情
    pub operation_detail: String,
    /// IP 地址
    pub ip: Option<String>,
    /// 设备信息
    pub device_info: Option<String>,
    /// 扩展信息
    pub extend_info: Option<serde_json::Value>,
}

/// 行为审计日志响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogGetResponse {
    /// 总数量
    pub total: i32,
    /// 当前页
    pub page: i32,
    /// 页面大小
    pub page_size: i32,
    /// 日志项列表
    pub items: Vec<AuditLogItem>,
}

// 默认实现
impl Default for OpenapiLogListRequest {
    fn default() -> Self {
        Self {
            page_token: None,
            page_size: Some(100),
            start_time: None,
            end_time: None,
            app_ids: None,
            apis: None,
            response_codes: None,
        }
    }
}

impl Default for AuditLogGetRequest {
    fn default() -> Self {
        Self {
            data_type: "all".to_string(),
            start_time: 0,
            end_time: 0,
            page: Some(1),
            page_size: Some(100),
            audit_types: None,
            operator_ids: None,
            object_ids: None,
        }
    }
}

// ApiResponseTrait 实现
impl ApiResponseTrait for OpenapiLogListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for AuditLogGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
