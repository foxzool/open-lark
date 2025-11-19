use openlark_core::api::{ApiResponseTrait, ResponseFormat};
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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_openapi_log_list_request() {
        let request = OpenapiLogListRequest {
            page_token: Some("token_123".to_string()),
            page_size: Some(50),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            app_ids: Some("app1,app2,app3".to_string()),
            apis: Some("/open-apis/user/v1/list,/open-apis/message/v4/send".to_string()),
            response_codes: Some("200,400,500".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("token_123"));
        assert!(json.contains("\"page_size\":50"));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("app1,app2,app3"));
        assert!(json.contains("/open-apis/user/v1/list"));
        assert!(json.contains("200,400,500"));
    }

    #[test]
    fn test_openapi_log_item() {
        let log_item = OpenapiLogItem {
            timestamp: 1640995200000,
            app_id: "cli_a1b2c3d4e5f6g7h8".to_string(),
            app_name: "测试应用".to_string(),
            api: "/open-apis/im/v1/messages".to_string(),
            method: "POST".to_string(),
            request_id: "req_12345678".to_string(),
            response_code: 200,
            response_time: 125,
            user_id: Some("ou_1234567890abcdef".to_string()),
            tenant_key: Some("0123456789abcdef".to_string()),
            ip: Some("192.168.1.100".to_string()),
            user_agent: Some("OpenLarkSDK/1.0.0".to_string()),
        };
        let json = serde_json::to_string(&log_item).unwrap();
        assert!(json.contains("1640995200000"));
        assert!(json.contains("cli_a1b2c3d4e5f6g7h8"));
        assert!(json.contains("测试应用"));
        assert!(json.contains("/open-apis/im/v1/messages"));
        assert!(json.contains("POST"));
        assert!(json.contains("req_12345678"));
        assert!(json.contains("\"response_code\":200"));
        assert!(json.contains("\"response_time\":125"));
        assert!(json.contains("ou_1234567890abcdef"));
        assert!(json.contains("192.168.1.100"));
        assert!(json.contains("OpenLarkSDK/1.0.0"));
    }

    #[test]
    fn test_openapi_log_list_response() {
        let log_item = OpenapiLogItem {
            timestamp: 1640995200000,
            app_id: "cli_test".to_string(),
            app_name: "Test App".to_string(),
            api: "/open-apis/contact/v3/users".to_string(),
            method: "GET".to_string(),
            request_id: "req_get_users".to_string(),
            response_code: 200,
            response_time: 85,
            user_id: Some("ou_test_user".to_string()),
            tenant_key: Some("tenant_test".to_string()),
            ip: Some("10.0.0.1".to_string()),
            user_agent: Some("Browser/Chrome".to_string()),
        };

        let response = OpenapiLogListResponse {
            has_more: true,
            page_token: Some("next_page_token".to_string()),
            items: vec![log_item],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"has_more\":true"));
        assert!(json.contains("next_page_token"));
        assert!(json.contains("cli_test"));
        assert!(json.contains("/open-apis/contact/v3/users"));
        assert!(json.contains("GET"));
    }

    #[test]
    fn test_audit_log_get_request() {
        let request = AuditLogGetRequest {
            data_type: "all".to_string(),
            start_time: 1640995200000,
            end_time: 1641081600000,
            page: Some(2),
            page_size: Some(200),
            audit_types: Some(vec![
                "login".to_string(),
                "file_access".to_string(),
                "admin_operation".to_string(),
            ]),
            operator_ids: Some(vec!["user_admin".to_string(), "user_manager".to_string()]),
            object_ids: Some(vec!["file_001".to_string(), "folder_002".to_string()]),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"data_type\":\"all\""));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("1641081600000"));
        assert!(json.contains("\"page\":2"));
        assert!(json.contains("\"page_size\":200"));
        assert!(json.contains("login"));
        assert!(json.contains("file_access"));
        assert!(json.contains("admin_operation"));
        assert!(json.contains("user_admin"));
        assert!(json.contains("file_001"));
    }

    #[test]
    fn test_audit_log_item() {
        let extend_info = serde_json::json!({
            "old_value": "admin",
            "new_value": "user",
            "affected_fields": ["role", "permissions"]
        });

        let log_item = AuditLogItem {
            log_id: "audit_log_123456".to_string(),
            timestamp: 1640995200000,
            audit_type: "user_role_change".to_string(),
            operator_id: "admin_001".to_string(),
            operator_name: "系统管理员".to_string(),
            object_id: Some("user_target_001".to_string()),
            object_name: Some("张三".to_string()),
            operation_detail: "将用户角色从管理员降级为普通用户".to_string(),
            ip: Some("172.16.0.100".to_string()),
            device_info: Some("Windows 10, Chrome 96.0.4664.110".to_string()),
            extend_info: Some(extend_info),
        };

        let json = serde_json::to_string(&log_item).unwrap();
        assert!(json.contains("audit_log_123456"));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("user_role_change"));
        assert!(json.contains("admin_001"));
        assert!(json.contains("系统管理员"));
        assert!(json.contains("user_target_001"));
        assert!(json.contains("张三"));
        assert!(json.contains("将用户角色从管理员降级为普通用户"));
        assert!(json.contains("172.16.0.100"));
        assert!(json.contains("Windows 10"));
        assert!(json.contains("old_value"));
        assert!(json.contains("new_value"));
        assert!(json.contains("affected_fields"));
    }

    #[test]
    fn test_audit_log_get_response() {
        let log_item1 = AuditLogItem {
            log_id: "log_001".to_string(),
            timestamp: 1640995200000,
            audit_type: "login".to_string(),
            operator_id: "user_001".to_string(),
            operator_name: "Alice".to_string(),
            object_id: None,
            object_name: None,
            operation_detail: "用户登录系统".to_string(),
            ip: Some("192.168.1.50".to_string()),
            device_info: Some("macOS, Safari 15.1".to_string()),
            extend_info: None,
        };

        let log_item2 = AuditLogItem {
            log_id: "log_002".to_string(),
            timestamp: 1640995260000,
            audit_type: "file_download".to_string(),
            operator_id: "user_001".to_string(),
            operator_name: "Alice".to_string(),
            object_id: Some("file_important.pdf".to_string()),
            object_name: Some("重要文档.pdf".to_string()),
            operation_detail: "下载敏感文件".to_string(),
            ip: Some("192.168.1.50".to_string()),
            device_info: Some("macOS, Safari 15.1".to_string()),
            extend_info: Some(
                serde_json::json!({"file_size": 2048576, "classification": "confidential"}),
            ),
        };

        let response = AuditLogGetResponse {
            total: 1250,
            page: 1,
            page_size: 100,
            items: vec![log_item1, log_item2],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"total\":1250"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"page_size\":100"));
        assert!(json.contains("log_001"));
        assert!(json.contains("log_002"));
        assert!(json.contains("login"));
        assert!(json.contains("file_download"));
        assert!(json.contains("用户登录系统"));
        assert!(json.contains("下载敏感文件"));
        assert!(json.contains("file_important.pdf"));
        assert!(json.contains("重要文档.pdf"));
        assert!(json.contains("confidential"));
    }

    #[test]
    fn test_default_implementations() {
        let default_openapi_request = OpenapiLogListRequest::default();
        assert_eq!(default_openapi_request.page_token, None);
        assert_eq!(default_openapi_request.page_size, Some(100));
        assert_eq!(default_openapi_request.start_time, None);
        assert_eq!(default_openapi_request.end_time, None);
        assert_eq!(default_openapi_request.app_ids, None);
        assert_eq!(default_openapi_request.apis, None);
        assert_eq!(default_openapi_request.response_codes, None);

        let default_audit_request = AuditLogGetRequest::default();
        assert_eq!(default_audit_request.data_type, "all");
        assert_eq!(default_audit_request.start_time, 0);
        assert_eq!(default_audit_request.end_time, 0);
        assert_eq!(default_audit_request.page, Some(1));
        assert_eq!(default_audit_request.page_size, Some(100));
        assert_eq!(default_audit_request.audit_types, None);
        assert_eq!(default_audit_request.operator_ids, None);
        assert_eq!(default_audit_request.object_ids, None);
    }

    #[test]
    fn test_api_response_trait() {
        assert_eq!(OpenapiLogListResponse::data_format(), ResponseFormat::Data);
        assert_eq!(AuditLogGetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_openapi_request = OpenapiLogListRequest {
            page_token: None,
            page_size: None,
            start_time: None,
            end_time: None,
            app_ids: None,
            apis: None,
            response_codes: None,
        };
        let json = serde_json::to_string(&minimal_openapi_request).unwrap();
        // Since the struct doesn't have skip_serializing_if, None values appear as null
        assert!(json.contains("\"page_token\":null"));
        assert!(json.contains("\"page_size\":null"));
        assert!(json.contains("\"start_time\":null"));

        // Test partial struct
        let partial_request = OpenapiLogListRequest {
            page_token: Some("test_token".to_string()),
            page_size: None,
            start_time: None,
            end_time: None,
            app_ids: None,
            apis: None,
            response_codes: None,
        };
        let json = serde_json::to_string(&partial_request).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("\"page_size\":null"));

        let minimal_openapi_item = OpenapiLogItem {
            timestamp: 1640995200000,
            app_id: "minimal_app".to_string(),
            app_name: "Minimal App".to_string(),
            api: "/api/test".to_string(),
            method: "GET".to_string(),
            request_id: "req_minimal".to_string(),
            response_code: 200,
            response_time: 50,
            user_id: None,
            tenant_key: None,
            ip: None,
            user_agent: None,
        };
        let json = serde_json::to_string(&minimal_openapi_item).unwrap();
        assert!(json.contains("minimal_app"));
        assert!(json.contains("\"user_id\":null"));
        assert!(json.contains("\"tenant_key\":null"));

        let minimal_audit_item = AuditLogItem {
            log_id: "minimal_log".to_string(),
            timestamp: 1640995200000,
            audit_type: "test".to_string(),
            operator_id: "op_001".to_string(),
            operator_name: "Operator".to_string(),
            object_id: None,
            object_name: None,
            operation_detail: "Test operation".to_string(),
            ip: None,
            device_info: None,
            extend_info: None,
        };
        let json = serde_json::to_string(&minimal_audit_item).unwrap();
        assert!(json.contains("minimal_log"));
        assert!(json.contains("Test operation"));
        assert!(json.contains("\"object_id\":null"));
        assert!(json.contains("\"ip\":null"));
    }

    #[test]
    fn test_complex_security_scenario() {
        // Simulate a complex security audit scenario
        let security_incident_extend_info = serde_json::json!({
            "incident_type": "unauthorized_access",
            "severity": "high",
            "affected_resources": ["user_database", "config_files"],
            "remediation_actions": ["password_reset", "access_revoked", "logs_preserved"],
            "investigation_status": "ongoing"
        });

        let security_log = AuditLogItem {
            log_id: "sec_incident_20240101_001".to_string(),
            timestamp: 1704067200000,
            audit_type: "security_incident".to_string(),
            operator_id: "security_system".to_string(),
            operator_name: "安全监控系统".to_string(),
            object_id: Some("suspicious_user_001".to_string()),
            object_name: Some("可疑用户账号".to_string()),
            operation_detail: "检测到未授权访问尝试，已自动触发安全响应流程".to_string(),
            ip: Some("203.0.113.42".to_string()),
            device_info: Some("Unknown Device, Tor Browser".to_string()),
            extend_info: Some(security_incident_extend_info),
        };

        let openapi_abuse_log = OpenapiLogItem {
            timestamp: 1704067200000,
            app_id: "cli_suspicious_app".to_string(),
            app_name: "可疑应用".to_string(),
            api: "/open-apis/contact/v3/users/batch_get".to_string(),
            method: "POST".to_string(),
            request_id: "req_batch_abuse_001".to_string(),
            response_code: 429,  // Rate limited
            response_time: 5000, // Very slow due to rate limiting
            user_id: Some("ou_suspicious_user".to_string()),
            tenant_key: Some("tenant_target".to_string()),
            ip: Some("203.0.113.42".to_string()),
            user_agent: Some("CustomScript/1.0 (Automated)".to_string()),
        };

        let security_response = AuditLogGetResponse {
            total: 1,
            page: 1,
            page_size: 100,
            items: vec![security_log],
        };

        let openapi_response = OpenapiLogListResponse {
            has_more: false,
            page_token: None,
            items: vec![openapi_abuse_log],
        };

        let security_json = serde_json::to_string(&security_response).unwrap();
        let openapi_json = serde_json::to_string(&openapi_response).unwrap();

        // Verify security incident logging
        assert!(security_json.contains("sec_incident_20240101_001"));
        assert!(security_json.contains("security_incident"));
        assert!(security_json.contains("安全监控系统"));
        assert!(security_json.contains("unauthorized_access"));
        assert!(security_json.contains("high"));
        assert!(security_json.contains("user_database"));
        assert!(security_json.contains("password_reset"));
        assert!(security_json.contains("ongoing"));

        // Verify API abuse logging
        assert!(openapi_json.contains("cli_suspicious_app"));
        assert!(openapi_json.contains("batch_get"));
        assert!(openapi_json.contains("\"response_code\":429"));
        assert!(openapi_json.contains("\"response_time\":5000"));
        assert!(openapi_json.contains("CustomScript/1.0"));
        assert!(openapi_json.contains("203.0.113.42"));
    }
}
