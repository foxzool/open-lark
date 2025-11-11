// passport模块的数据模型定义
use open_lark_core::api_resp::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 用户会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    /// 用户邮箱
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// 用户手机号
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    /// 头像
    #[serde(rename = "avatar")]
    pub avatar: Option<String>,
    /// 部门信息
    #[serde(rename = "department")]
    pub department: Option<DepartmentInfo>,
    /// 登录时间
    #[serde(rename = "login_time")]
    pub login_time: String,
    /// 登录设备
    #[serde(rename = "device_info")]
    pub device_info: Option<DeviceInfo>,
    /// 登录IP
    #[serde(rename = "login_ip")]
    pub login_ip: Option<String>,
    /// 会话状态
    #[serde(rename = "session_status")]
    pub session_status: SessionStatus,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentInfo {
    /// 部门ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 部门名称
    #[serde(rename = "department_name")]
    pub department_name: String,
    /// 部门路径
    #[serde(rename = "department_path")]
    pub department_path: Option<String>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型
    #[serde(rename = "device_type")]
    pub device_type: String,
    /// 设备型号
    #[serde(rename = "device_model")]
    pub device_model: Option<String>,
    /// 操作系统
    #[serde(rename = "os")]
    pub os: Option<String>,
    /// 浏览器信息
    #[serde(rename = "browser")]
    pub browser: Option<String>,
}

/// 会话状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// 活跃
    Active,
    /// 已过期
    Expired,
    /// 已登出
    LoggedOut,
}

/// 批量会话查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryRequest {
    /// 用户ID列表
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
    /// 用户ID类型
    #[serde(rename = "user_id_type")]
    pub user_id_type: String,
}

/// 批量会话查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryResponse {
    /// 用户会话列表
    #[serde(rename = "user_sessions")]
    pub user_sessions: Vec<UserSession>,
    /// 失败的用户ID列表
    #[serde(rename = "failed_user_ids")]
    pub failed_user_ids: Vec<String>,
}

/// 退出登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// 用户ID列表
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
    /// 用户ID类型
    #[serde(rename = "user_id_type")]
    pub user_id_type: String,
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogoutResponse {
    /// 成功退出的用户数量
    #[serde(rename = "success_count", default)]
    pub success_count: i32,
    /// 失败的用户数量
    #[serde(rename = "failed_count", default)]
    pub failed_count: i32,
    /// 失败的用户ID列表
    #[serde(rename = "failed_user_ids", default)]
    pub failed_user_ids: Vec<String>,
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

impl<T> ApiResponseTrait for PassportResponse<T>
where
    T: for<'a> serde::Deserialize<'a> + serde::Serialize + Send + Sync + std::fmt::Debug + 'static,
{
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logout_request_serialization() {
        let request = LogoutRequest {
            user_ids: vec!["user_123".to_string(), "user_456".to_string()],
            user_id_type: "open_id".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: LogoutRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.user_ids, deserialized.user_ids);
        assert_eq!(request.user_id_type, deserialized.user_id_type);
    }

    #[test]
    fn test_logout_response_default() {
        let response = LogoutResponse::default();
        assert_eq!(response.success_count, 0);
        assert_eq!(response.failed_count, 0);
        assert!(response.failed_user_ids.is_empty());
    }

    #[test]
    fn test_logout_response_creation() {
        let response = LogoutResponse {
            success_count: 2,
            failed_count: 1,
            failed_user_ids: vec!["user_invalid".to_string()],
        };

        assert_eq!(response.success_count, 2);
        assert_eq!(response.failed_count, 1);
        assert_eq!(response.failed_user_ids.len(), 1);
        assert_eq!(response.failed_user_ids[0], "user_invalid");
    }

    #[test]
    fn test_session_status_serialization() {
        let status = SessionStatus::Active;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"active\"");

        let deserialized: SessionStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, SessionStatus::Active);
    }

    #[test]
    fn test_passport_response_with_data() {
        let logout_response = LogoutResponse {
            success_count: 1,
            failed_count: 0,
            failed_user_ids: vec![],
        };

        let passport_response = PassportResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(logout_response),
        };

        let serialized = serde_json::to_string(&passport_response).unwrap();
        let deserialized: PassportResponse<LogoutResponse> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.code, 0);
        assert_eq!(deserialized.msg, "success");
        assert!(deserialized.data.is_some());
        assert_eq!(deserialized.data.unwrap().success_count, 1);
    }
}
