use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，下次请求的起点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 数据项目列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
}

// ============ 登录密码管理相关结构 ============

/// 重置用户企业邮箱密码请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    /// 用户ID，ID类型与查询参数中的user_id_type对应
    pub user_id: String,
    /// 新密码
    pub password: String,
}

/// 重置用户企业邮箱密码响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetResponse {
    /// 重置结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 操作时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<String>,
}

// ============ 数据报表管理相关结构 ============

/// 部门维度数据报表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentDataReportRequest {
    /// 开始日期，格式为YYYY-MM-DD
    pub start_date: String,
    /// 结束日期，格式为YYYY-MM-DD
    pub end_date: String,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 用户维度数据报表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataReportRequest {
    /// 开始日期，格式为YYYY-MM-DD
    pub start_date: String,
    /// 结束日期，格式为YYYY-MM-DD
    pub end_date: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据报表信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataReport {
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 用户ID或部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户名或部门名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 活跃用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i32>,
    /// 新增用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_users: Option<i32>,
    /// 消息发送数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<i32>,
    /// 云文档使用数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_usage: Option<i32>,
    /// 会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_duration: Option<i32>,
    /// 其他统计数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics: Option<HashMap<String, Value>>,
}

// ============ 企业勋章相关结构 ============

/// 勋章创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeCreateRequest {
    /// 勋章名称
    pub name: String,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 勋章更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeUpdateRequest {
    /// 勋章ID
    pub badge_id: String,
    /// 勋章名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 勋章图片上传请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeImageUploadRequest {
    /// 图片文件内容（base64编码或文件流）
    pub image: String,
    /// 图片文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// 勋章列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeListRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 勋章名称（模糊搜索）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 勋章查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGetRequest {
    /// 勋章ID
    pub badge_id: String,
}

/// 勋章信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 勋章名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 图片上传结果
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeImageUploadResult {
    /// 图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

// ============ 勋章授予名单相关结构 ============

/// 勋章授予名单创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantCreateRequest {
    /// 勋章ID
    pub badge_id: String,
    /// 授予名单名称
    pub name: String,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    pub user_list: Vec<BadgeGrantUser>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 勋章授予名单更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantUpdateRequest {
    /// 授予名单ID
    pub grant_id: String,
    /// 授予名单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<BadgeGrantUser>>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 勋章授予名单删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantDeleteRequest {
    /// 授予名单ID
    pub grant_id: String,
}

/// 勋章授予名单列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantListRequest {
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 授予名单名称（模糊搜索）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 勋章授予名单查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantGetRequest {
    /// 授予名单ID
    pub grant_id: String,
}

/// 勋章授予用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantUser {
    /// 用户ID
    pub user_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 授予理由
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 授予时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_time: Option<String>,
}

/// 勋章授予名单信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrant {
    /// 授予名单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 授予名单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<BadgeGrantUser>>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page_response = PageResponse {
            has_more: Some(true),
            page_token: Some("next_page_token".to_string()),
            items: Some(vec!["item1".to_string(), "item2".to_string()]),
        };
        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.items, deserialized.items);
    }

    #[test]
    fn test_page_response_with_none_values() {
        let page_response: PageResponse<String> = PageResponse {
            has_more: None,
            page_token: None,
            items: None,
        };
        let serialized = serde_json::to_string(&page_response).unwrap();
        assert!(!serialized.contains("has_more"));
        assert!(!serialized.contains("page_token"));
        assert!(!serialized.contains("items"));
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.items, deserialized.items);
    }

    #[test]
    fn test_password_reset_request_serialization() {
        let request = PasswordResetRequest {
            user_id: "user_123456".to_string(),
            password: "new_secure_password".to_string(),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: PasswordResetRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.user_id, deserialized.user_id);
        assert_eq!(request.password, deserialized.password);
    }

    #[test]
    fn test_password_reset_request_default() {
        let request = PasswordResetRequest::default();
        assert_eq!(request.user_id, "");
        assert_eq!(request.password, "");
    }

    #[test]
    fn test_password_reset_response_serialization() {
        let response = PasswordResetResponse {
            success: Some(true),
            reset_time: Some("2022-01-01T10:00:00Z".to_string()),
        };
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: PasswordResetResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.reset_time, deserialized.reset_time);
    }

    #[test]
    fn test_department_data_report_request_serialization() {
        let request = DepartmentDataReportRequest {
            start_date: "2022-01-01".to_string(),
            end_date: "2022-01-31".to_string(),
            department_id_type: Some("open_department_id".to_string()),
            department_id: Some("dept_123".to_string()),
            page_size: Some(50),
            page_token: Some("token_abc".to_string()),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: DepartmentDataReportRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.start_date, deserialized.start_date);
        assert_eq!(request.end_date, deserialized.end_date);
        assert_eq!(request.department_id_type, deserialized.department_id_type);
        assert_eq!(request.department_id, deserialized.department_id);
        assert_eq!(request.page_size, deserialized.page_size);
    }

    #[test]
    fn test_user_data_report_request_serialization() {
        let request = UserDataReportRequest {
            start_date: "2022-02-01".to_string(),
            end_date: "2022-02-28".to_string(),
            user_id_type: Some("open_id".to_string()),
            user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
            department_id_type: Some("department_id".to_string()),
            department_id: Some("dept_456".to_string()),
            page_size: Some(100),
            page_token: Some("token_xyz".to_string()),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UserDataReportRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.start_date, deserialized.start_date);
        assert_eq!(request.end_date, deserialized.end_date);
        assert_eq!(request.user_id_type, deserialized.user_id_type);
        assert_eq!(request.user_ids, deserialized.user_ids);
    }

    #[test]
    fn test_data_report_serialization() {
        let mut additional_metrics = HashMap::new();
        additional_metrics.insert(
            "custom_metric_1".to_string(),
            serde_json::Value::Number(serde_json::Number::from(42)),
        );
        additional_metrics.insert(
            "custom_metric_2".to_string(),
            serde_json::Value::String("test_value".to_string()),
        );

        let report = DataReport {
            date: Some("2022-01-15".to_string()),
            id: Some("report_123".to_string()),
            name: Some("技术部".to_string()),
            active_users: Some(150),
            new_users: Some(10),
            messages_sent: Some(2500),
            docs_usage: Some(300),
            meeting_duration: Some(720),
            additional_metrics: Some(additional_metrics),
        };
        let serialized = serde_json::to_string(&report).unwrap();
        let deserialized: DataReport = serde_json::from_str(&serialized).unwrap();
        assert_eq!(report.date, deserialized.date);
        assert_eq!(report.id, deserialized.id);
        assert_eq!(report.name, deserialized.name);
        assert_eq!(report.active_users, deserialized.active_users);
        assert_eq!(report.new_users, deserialized.new_users);
        assert_eq!(report.messages_sent, deserialized.messages_sent);
        assert_eq!(report.docs_usage, deserialized.docs_usage);
        assert_eq!(report.meeting_duration, deserialized.meeting_duration);
    }

    #[test]
    fn test_badge_create_request_serialization() {
        let mut i18n_name = HashMap::new();
        i18n_name.insert("zh_cn".to_string(), "优秀员工".to_string());
        i18n_name.insert("en_us".to_string(), "Excellent Employee".to_string());

        let mut i18n_description = HashMap::new();
        i18n_description.insert("zh_cn".to_string(), "表彰优秀表现的员工".to_string());
        i18n_description.insert(
            "en_us".to_string(),
            "Recognizing excellent performance".to_string(),
        );

        let request = BadgeCreateRequest {
            name: "优秀员工徽章".to_string(),
            description: Some("表彰优秀表现".to_string()),
            detail_description: Some("详细的表彰说明".to_string()),
            show_detail_time: Some(true),
            image_key: Some("badge_image_key_123".to_string()),
            i18n_name: Some(i18n_name),
            i18n_description: Some(i18n_description),
            i18n_detail_description: None,
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BadgeCreateRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.detail_description, deserialized.detail_description);
        assert_eq!(request.show_detail_time, deserialized.show_detail_time);
        assert_eq!(request.image_key, deserialized.image_key);
    }

    #[test]
    fn test_badge_update_request_serialization() {
        let request = BadgeUpdateRequest {
            badge_id: "badge_456".to_string(),
            name: Some("更新后的徽章".to_string()),
            description: Some("更新后的描述".to_string()),
            detail_description: None,
            show_detail_time: Some(false),
            image_key: None,
            i18n_name: None,
            i18n_description: None,
            i18n_detail_description: None,
        };
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("detail_description"));
        assert!(!serialized.contains("image_key"));
        assert!(!serialized.contains("i18n_name"));
        let deserialized: BadgeUpdateRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.badge_id, deserialized.badge_id);
        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
    }

    #[test]
    fn test_badge_image_upload_request_serialization() {
        let request = BadgeImageUploadRequest {
            image: "base64_encoded_image_data".to_string(),
            file_name: Some("badge_icon.png".to_string()),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BadgeImageUploadRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.image, deserialized.image);
        assert_eq!(request.file_name, deserialized.file_name);
    }

    #[test]
    fn test_badge_list_request_serialization() {
        let request = BadgeListRequest {
            page_size: Some(20),
            page_token: Some("list_token_123".to_string()),
            name: Some("员工".to_string()),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BadgeListRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.name, deserialized.name);
    }

    #[test]
    fn test_badge_get_request_serialization() {
        let request = BadgeGetRequest {
            badge_id: "get_badge_789".to_string(),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BadgeGetRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.badge_id, deserialized.badge_id);
    }

    #[test]
    fn test_badge_serialization() {
        let mut i18n_name = HashMap::new();
        i18n_name.insert("zh_cn".to_string(), "年度最佳".to_string());
        i18n_name.insert("en_us".to_string(), "Annual Best".to_string());

        let badge = Badge {
            badge_id: Some("badge_001".to_string()),
            name: Some("年度最佳员工".to_string()),
            description: Some("年度表现优异的员工".to_string()),
            detail_description: Some("详细的年度表彰说明".to_string()),
            show_detail_time: Some(true),
            image_url: Some("https://example.com/badge.png".to_string()),
            image_key: Some("annual_badge_key".to_string()),
            create_time: Some("2022-01-01T00:00:00Z".to_string()),
            update_time: Some("2022-06-01T00:00:00Z".to_string()),
            creator_id: Some("creator_123".to_string()),
            i18n_name: Some(i18n_name),
            i18n_description: None,
            i18n_detail_description: None,
        };
        let serialized = serde_json::to_string(&badge).unwrap();
        let deserialized: Badge = serde_json::from_str(&serialized).unwrap();
        assert_eq!(badge.badge_id, deserialized.badge_id);
        assert_eq!(badge.name, deserialized.name);
        assert_eq!(badge.description, deserialized.description);
        assert_eq!(badge.show_detail_time, deserialized.show_detail_time);
        assert_eq!(badge.image_url, deserialized.image_url);
        assert_eq!(badge.create_time, deserialized.create_time);
    }

    #[test]
    fn test_badge_image_upload_result_serialization() {
        let result = BadgeImageUploadResult {
            image_key: Some("uploaded_key_456".to_string()),
            image_url: Some("https://example.com/uploaded_badge.png".to_string()),
        };
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: BadgeImageUploadResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result.image_key, deserialized.image_key);
        assert_eq!(result.image_url, deserialized.image_url);
    }

    #[test]
    fn test_badge_grant_create_request_serialization() {
        let grant_users = vec![
            BadgeGrantUser {
                user_id: "user_1".to_string(),
                user_id_type: Some("open_id".to_string()),
                reason: Some("优秀表现".to_string()),
                grant_time: Some("2022-03-01T10:00:00Z".to_string()),
            },
            BadgeGrantUser {
                user_id: "user_2".to_string(),
                user_id_type: Some("user_id".to_string()),
                reason: Some("杰出贡献".to_string()),
                grant_time: Some("2022-03-01T10:00:00Z".to_string()),
            },
        ];

        let request = BadgeGrantCreateRequest {
            badge_id: "grant_badge_123".to_string(),
            name: "Q1优秀员工名单".to_string(),
            description: Some("第一季度优秀员工表彰".to_string()),
            user_list: grant_users,
            effective_time: Some("2022-03-01T00:00:00Z".to_string()),
            expiry_time: Some("2022-06-01T00:00:00Z".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BadgeGrantCreateRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.badge_id, deserialized.badge_id);
        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.user_list.len(), deserialized.user_list.len());
        assert_eq!(request.effective_time, deserialized.effective_time);
        assert_eq!(request.expiry_time, deserialized.expiry_time);
        assert_eq!(request.time_zone, deserialized.time_zone);
    }

    #[test]
    fn test_badge_grant_user_serialization() {
        let grant_user = BadgeGrantUser {
            user_id: "grant_user_789".to_string(),
            user_id_type: Some("union_id".to_string()),
            reason: Some("项目领导能力突出".to_string()),
            grant_time: Some("2022-04-15T14:30:00Z".to_string()),
        };
        let serialized = serde_json::to_string(&grant_user).unwrap();
        let deserialized: BadgeGrantUser = serde_json::from_str(&serialized).unwrap();
        assert_eq!(grant_user.user_id, deserialized.user_id);
        assert_eq!(grant_user.user_id_type, deserialized.user_id_type);
        assert_eq!(grant_user.reason, deserialized.reason);
        assert_eq!(grant_user.grant_time, deserialized.grant_time);
    }

    #[test]
    fn test_badge_grant_serialization() {
        let grant = BadgeGrant {
            grant_id: Some("grant_001".to_string()),
            badge_id: Some("badge_002".to_string()),
            name: Some("Q2卓越团队".to_string()),
            description: Some("第二季度卓越团队表彰".to_string()),
            user_list: Some(vec![BadgeGrantUser {
                user_id: "team_lead".to_string(),
                user_id_type: Some("employee_id".to_string()),
                reason: Some("团队管理优秀".to_string()),
                grant_time: Some("2022-07-01T09:00:00Z".to_string()),
            }]),
            effective_time: Some("2022-07-01T00:00:00Z".to_string()),
            expiry_time: Some("2022-10-01T00:00:00Z".to_string()),
            time_zone: Some("UTC".to_string()),
            create_time: Some("2022-06-15T12:00:00Z".to_string()),
            update_time: Some("2022-06-20T15:30:00Z".to_string()),
            creator_id: Some("admin_user_456".to_string()),
        };
        let serialized = serde_json::to_string(&grant).unwrap();
        let deserialized: BadgeGrant = serde_json::from_str(&serialized).unwrap();
        assert_eq!(grant.grant_id, deserialized.grant_id);
        assert_eq!(grant.badge_id, deserialized.badge_id);
        assert_eq!(grant.name, deserialized.name);
        assert_eq!(grant.description, deserialized.description);
        assert_eq!(grant.effective_time, deserialized.effective_time);
        assert_eq!(grant.time_zone, deserialized.time_zone);
        assert_eq!(grant.creator_id, deserialized.creator_id);
    }

    #[test]
    fn test_badge_grant_with_none_values() {
        let grant = BadgeGrant {
            grant_id: Some("minimal_grant".to_string()),
            badge_id: None,
            name: None,
            description: None,
            user_list: None,
            effective_time: None,
            expiry_time: None,
            time_zone: None,
            create_time: None,
            update_time: None,
            creator_id: None,
        };
        let serialized = serde_json::to_string(&grant).unwrap();
        assert!(!serialized.contains("badge_id"));
        assert!(!serialized.contains("name"));
        assert!(!serialized.contains("description"));
        assert!(!serialized.contains("user_list"));
        assert!(!serialized.contains("effective_time"));
        let deserialized: BadgeGrant = serde_json::from_str(&serialized).unwrap();
        assert_eq!(grant.grant_id, deserialized.grant_id);
        assert_eq!(grant.badge_id, deserialized.badge_id);
    }

    #[test]
    fn test_complex_data_report_with_additional_metrics() {
        let mut complex_metrics = HashMap::new();
        complex_metrics.insert(
            "video_calls".to_string(),
            serde_json::Value::Number(serde_json::Number::from(45)),
        );
        complex_metrics.insert(
            "approval_rate".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(95.5).unwrap()),
        );
        complex_metrics.insert(
            "top_feature".to_string(),
            serde_json::Value::String("collaboration".to_string()),
        );
        complex_metrics.insert(
            "enabled_features".to_string(),
            serde_json::Value::Array(vec![
                serde_json::Value::String("chat".to_string()),
                serde_json::Value::String("docs".to_string()),
                serde_json::Value::String("calendar".to_string()),
            ]),
        );

        let report = DataReport {
            date: Some("2022-12-31".to_string()),
            id: Some("annual_report_2022".to_string()),
            name: Some("全公司年度报告".to_string()),
            active_users: Some(5000),
            new_users: Some(500),
            messages_sent: Some(1000000),
            docs_usage: Some(50000),
            meeting_duration: Some(720000),
            additional_metrics: Some(complex_metrics),
        };
        let serialized = serde_json::to_string(&report).unwrap();
        let deserialized: DataReport = serde_json::from_str(&serialized).unwrap();
        assert_eq!(report.date, deserialized.date);
        assert_eq!(report.id, deserialized.id);
        assert_eq!(report.name, deserialized.name);
        assert_eq!(report.active_users, deserialized.active_users);
        assert_eq!(report.meeting_duration, deserialized.meeting_duration);

        // Verify additional metrics are properly deserialized
        if let (Some(orig_metrics), Some(deser_metrics)) =
            (&report.additional_metrics, &deserialized.additional_metrics)
        {
            assert_eq!(orig_metrics.len(), deser_metrics.len());
            assert!(deser_metrics.contains_key("video_calls"));
            assert!(deser_metrics.contains_key("approval_rate"));
            assert!(deser_metrics.contains_key("top_feature"));
            assert!(deser_metrics.contains_key("enabled_features"));
        }
    }
}
