use serde::{Deserialize, Serialize};

/// 系统状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    /// 系统状态ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status_id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 是否开启，true-开启，false-关闭
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 国际化内容
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nContent {
    /// 中文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 创建系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSystemStatusRequest {
    /// 标题
    pub title: String,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高，默认为1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 更新系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSystemStatusRequest {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 国际化标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<I18nContent>,
    /// 图标样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<String>,
    /// 图标链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 优先级，数字越小，优先级越高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 批量操作系统状态请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSystemStatusRequest {
    /// 系统状态ID列表
    pub system_status_ids: Vec<String>,
}

/// 系统状态列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSystemStatusRequest {
    /// 页码，从1开始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// 每页数量，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_system_status_serialization() {
        let status = SystemStatus {
            system_status_id: Some("status_123".to_string()),
            title: Some("系统维护通知".to_string()),
            i18n_title: Some(I18nContent {
                zh_cn: Some("系统维护通知".to_string()),
                en_us: Some("System Maintenance Notice".to_string()),
                ja_jp: Some("システムメンテナンス通知".to_string()),
            }),
            icon_style: Some("warning".to_string()),
            icon_url: Some("https://example.com/warning.png".to_string()),
            priority: Some(100),
            is_open: Some(true),
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1641081600000".to_string()),
        };

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: SystemStatus = serde_json::from_str(&serialized).unwrap();

        assert_eq!(status.system_status_id, deserialized.system_status_id);
        assert_eq!(status.title, deserialized.title);
        assert_eq!(status.icon_style, deserialized.icon_style);
        assert_eq!(status.icon_url, deserialized.icon_url);
        assert_eq!(status.priority, deserialized.priority);
        assert_eq!(status.is_open, deserialized.is_open);
        assert_eq!(status.create_time, deserialized.create_time);
        assert_eq!(status.update_time, deserialized.update_time);

        // Check nested i18n_title
        if let (Some(orig_i18n), Some(deser_i18n)) = (&status.i18n_title, &deserialized.i18n_title)
        {
            assert_eq!(orig_i18n.zh_cn, deser_i18n.zh_cn);
            assert_eq!(orig_i18n.en_us, deser_i18n.en_us);
            assert_eq!(orig_i18n.ja_jp, deser_i18n.ja_jp);
        }
    }

    #[test]
    fn test_system_status_with_none_values() {
        let status = SystemStatus {
            system_status_id: Some("minimal_status".to_string()),
            title: None,
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: None,
            is_open: None,
            create_time: None,
            update_time: None,
        };

        let serialized = serde_json::to_string(&status).unwrap();
        assert!(!serialized.contains("title"));
        assert!(!serialized.contains("i18n_title"));
        assert!(!serialized.contains("icon_style"));
        assert!(!serialized.contains("priority"));
        assert!(!serialized.contains("is_open"));

        let deserialized: SystemStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status.system_status_id, deserialized.system_status_id);
        assert!(deserialized.title.is_none());
        assert!(deserialized.i18n_title.is_none());
        assert!(deserialized.priority.is_none());
    }

    #[test]
    fn test_i18n_content_serialization() {
        let i18n = I18nContent {
            zh_cn: Some("中文标题".to_string()),
            en_us: Some("English Title".to_string()),
            ja_jp: Some("日本語タイトル".to_string()),
        };

        let serialized = serde_json::to_string(&i18n).unwrap();
        let deserialized: I18nContent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(i18n.zh_cn, deserialized.zh_cn);
        assert_eq!(i18n.en_us, deserialized.en_us);
        assert_eq!(i18n.ja_jp, deserialized.ja_jp);
    }

    #[test]
    fn test_i18n_content_partial_languages() {
        let i18n = I18nContent {
            zh_cn: Some("中文内容".to_string()),
            en_us: Some("English Content".to_string()),
            ja_jp: None,
        };

        let serialized = serde_json::to_string(&i18n).unwrap();
        assert!(!serialized.contains("ja_jp"));

        let deserialized: I18nContent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(i18n.zh_cn, deserialized.zh_cn);
        assert_eq!(i18n.en_us, deserialized.en_us);
        assert!(deserialized.ja_jp.is_none());
    }

    #[test]
    fn test_create_system_status_request_serialization() {
        let request = CreateSystemStatusRequest {
            title: "重要公告".to_string(),
            i18n_title: Some(I18nContent {
                zh_cn: Some("重要公告".to_string()),
                en_us: Some("Important Announcement".to_string()),
                ja_jp: None,
            }),
            icon_style: Some("info".to_string()),
            icon_url: Some("https://icons.example.com/info.svg".to_string()),
            priority: Some(50),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateSystemStatusRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.icon_style, deserialized.icon_style);
        assert_eq!(request.icon_url, deserialized.icon_url);
        assert_eq!(request.priority, deserialized.priority);

        // Check i18n_title
        if let (Some(orig_i18n), Some(deser_i18n)) = (&request.i18n_title, &deserialized.i18n_title)
        {
            assert_eq!(orig_i18n.zh_cn, deser_i18n.zh_cn);
            assert_eq!(orig_i18n.en_us, deser_i18n.en_us);
            assert_eq!(orig_i18n.ja_jp, deser_i18n.ja_jp);
        }
    }

    #[test]
    fn test_create_system_status_request_minimal() {
        let request = CreateSystemStatusRequest {
            title: "简单通知".to_string(),
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("i18n_title"));
        assert!(!serialized.contains("icon_style"));
        assert!(!serialized.contains("priority"));

        let deserialized: CreateSystemStatusRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.title, deserialized.title);
        assert!(deserialized.i18n_title.is_none());
        assert!(deserialized.priority.is_none());
    }

    #[test]
    fn test_update_system_status_request_serialization() {
        let request = UpdateSystemStatusRequest {
            title: Some("更新后的标题".to_string()),
            i18n_title: Some(I18nContent {
                zh_cn: Some("更新后的标题".to_string()),
                en_us: Some("Updated Title".to_string()),
                ja_jp: Some("更新されたタイトル".to_string()),
            }),
            icon_style: Some("success".to_string()),
            icon_url: Some("https://icons.example.com/success.svg".to_string()),
            priority: Some(25),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateSystemStatusRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.icon_style, deserialized.icon_style);
        assert_eq!(request.icon_url, deserialized.icon_url);
        assert_eq!(request.priority, deserialized.priority);

        // Check i18n_title
        if let (Some(orig_i18n), Some(deser_i18n)) = (&request.i18n_title, &deserialized.i18n_title)
        {
            assert_eq!(orig_i18n.zh_cn, deser_i18n.zh_cn);
            assert_eq!(orig_i18n.en_us, deser_i18n.en_us);
            assert_eq!(orig_i18n.ja_jp, deser_i18n.ja_jp);
        }
    }

    #[test]
    fn test_update_system_status_request_all_none() {
        let request = UpdateSystemStatusRequest {
            title: None,
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");

        let deserialized: UpdateSystemStatusRequest = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.title.is_none());
        assert!(deserialized.i18n_title.is_none());
        assert!(deserialized.icon_style.is_none());
        assert!(deserialized.icon_url.is_none());
        assert!(deserialized.priority.is_none());
    }

    #[test]
    fn test_batch_system_status_request_serialization() {
        let request = BatchSystemStatusRequest {
            system_status_ids: vec![
                "status_001".to_string(),
                "status_002".to_string(),
                "status_003".to_string(),
            ],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BatchSystemStatusRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.system_status_ids, deserialized.system_status_ids);
        assert_eq!(deserialized.system_status_ids.len(), 3);
        assert_eq!(deserialized.system_status_ids[0], "status_001");
        assert_eq!(deserialized.system_status_ids[1], "status_002");
        assert_eq!(deserialized.system_status_ids[2], "status_003");
    }

    #[test]
    fn test_batch_system_status_request_empty_list() {
        let request = BatchSystemStatusRequest {
            system_status_ids: vec![],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BatchSystemStatusRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.system_status_ids, deserialized.system_status_ids);
        assert_eq!(deserialized.system_status_ids.len(), 0);
        assert!(deserialized.system_status_ids.is_empty());
    }

    #[test]
    fn test_list_system_status_request_serialization() {
        let request = ListSystemStatusRequest {
            page: Some(2),
            page_size: Some(50),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ListSystemStatusRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.page, deserialized.page);
        assert_eq!(request.page_size, deserialized.page_size);
    }

    #[test]
    fn test_list_system_status_request_with_defaults() {
        let request = ListSystemStatusRequest {
            page: None,
            page_size: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("page"));
        assert!(!serialized.contains("page_size"));

        let deserialized: ListSystemStatusRequest = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.page.is_none());
        assert!(deserialized.page_size.is_none());
    }

    #[test]
    fn test_complex_system_status_scenario() {
        let complex_status = SystemStatus {
            system_status_id: Some("emergency_maintenance_2023".to_string()),
            title: Some("紧急系统维护".to_string()),
            i18n_title: Some(I18nContent {
                zh_cn: Some("紧急系统维护通知".to_string()),
                en_us: Some("Emergency System Maintenance Notice".to_string()),
                ja_jp: Some("緊急システムメンテナンス通知".to_string()),
            }),
            icon_style: Some("error".to_string()),
            icon_url: Some("https://cdn.company.com/icons/maintenance.svg".to_string()),
            priority: Some(1), // Highest priority
            is_open: Some(true),
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1641081600000".to_string()),
        };

        let serialized = serde_json::to_string(&complex_status).unwrap();
        let deserialized: SystemStatus = serde_json::from_str(&serialized).unwrap();

        // Verify all fields
        assert_eq!(
            complex_status.system_status_id,
            deserialized.system_status_id
        );
        assert_eq!(complex_status.title, deserialized.title);
        assert_eq!(complex_status.icon_style, deserialized.icon_style);
        assert_eq!(complex_status.icon_url, deserialized.icon_url);
        assert_eq!(complex_status.priority, deserialized.priority);
        assert_eq!(complex_status.is_open, deserialized.is_open);
        assert_eq!(complex_status.create_time, deserialized.create_time);
        assert_eq!(complex_status.update_time, deserialized.update_time);

        // Verify priority is highest
        assert_eq!(deserialized.priority.unwrap(), 1);
        assert!(deserialized.is_open.unwrap());

        // Verify i18n content
        if let Some(i18n) = &deserialized.i18n_title {
            assert!(i18n.zh_cn.is_some());
            assert!(i18n.en_us.is_some());
            assert!(i18n.ja_jp.is_some());
            assert!(i18n.zh_cn.as_ref().unwrap().contains("紧急"));
            assert!(i18n.en_us.as_ref().unwrap().contains("Emergency"));
            assert!(i18n.ja_jp.as_ref().unwrap().contains("緊急"));
        }
    }

    #[test]
    fn test_priority_ordering_scenarios() {
        // Test different priority values
        let high_priority = SystemStatus {
            system_status_id: Some("high".to_string()),
            title: Some("High Priority".to_string()),
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: Some(1),
            is_open: Some(true),
            create_time: None,
            update_time: None,
        };

        let medium_priority = SystemStatus {
            system_status_id: Some("medium".to_string()),
            title: Some("Medium Priority".to_string()),
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: Some(500),
            is_open: Some(true),
            create_time: None,
            update_time: None,
        };

        let low_priority = SystemStatus {
            system_status_id: Some("low".to_string()),
            title: Some("Low Priority".to_string()),
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: Some(1000),
            is_open: Some(false),
            create_time: None,
            update_time: None,
        };

        let high_serialized = serde_json::to_string(&high_priority).unwrap();
        let medium_serialized = serde_json::to_string(&medium_priority).unwrap();
        let low_serialized = serde_json::to_string(&low_priority).unwrap();

        let high_deserialized: SystemStatus = serde_json::from_str(&high_serialized).unwrap();
        let medium_deserialized: SystemStatus = serde_json::from_str(&medium_serialized).unwrap();
        let low_deserialized: SystemStatus = serde_json::from_str(&low_serialized).unwrap();

        // Verify priority ordering (lower number = higher priority)
        assert!(high_deserialized.priority.unwrap() < medium_deserialized.priority.unwrap());
        assert!(medium_deserialized.priority.unwrap() < low_deserialized.priority.unwrap());

        // Verify status states
        assert!(high_deserialized.is_open.unwrap());
        assert!(medium_deserialized.is_open.unwrap());
        assert!(!low_deserialized.is_open.unwrap());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let status = SystemStatus {
            system_status_id: Some("debug_test".to_string()),
            title: Some("Debug Test".to_string()),
            i18n_title: None,
            icon_style: None,
            icon_url: None,
            priority: None,
            is_open: None,
            create_time: None,
            update_time: None,
        };

        let debug_string = format!("{:?}", status);
        assert!(debug_string.contains("SystemStatus"));
        assert!(debug_string.contains("debug_test"));
        assert!(debug_string.contains("Debug Test"));
    }
}
