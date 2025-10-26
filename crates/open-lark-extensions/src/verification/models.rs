use serde::{Deserialize, Serialize};

/// 认证信息
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationInfo {
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_status: Option<String>,
    /// 认证状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    /// 认证类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_type: Option<String>,
    /// 认证时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_time: Option<String>,
    /// 过期时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// 租户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_info: Option<TenantInfo>,
}

/// 租户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantInfo {
    /// 租户key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    /// 租户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_info_creation() {
        let verification_info = VerificationInfo {
            app_id: Some("cli_test_app".to_string()),
            app_name: Some("Test CLI Application".to_string()),
            app_status: Some("active".to_string()),
            verification_status: Some("verified".to_string()),
            verification_type: Some("self_built".to_string()),
            verification_time: Some("1642723200000".to_string()),
            expire_time: Some("1674259200000".to_string()),
            scopes: Some(vec!["im:read".to_string(), "contact:read".to_string()]),
            tenant_info: Some(TenantInfo {
                tenant_key: Some("2ed263bf32cf1651".to_string()),
                tenant_name: Some("Test Organization".to_string()),
            }),
        };

        assert_eq!(verification_info.app_id, Some("cli_test_app".to_string()));
        assert_eq!(
            verification_info.app_name,
            Some("Test CLI Application".to_string())
        );
        assert_eq!(
            verification_info.verification_status,
            Some("verified".to_string())
        );
        assert_eq!(verification_info.scopes.as_ref().unwrap().len(), 2);
        assert!(verification_info.tenant_info.is_some());
    }

    #[test]
    fn test_verification_info_serialization() {
        let verification_info = VerificationInfo {
            app_id: Some("app_123".to_string()),
            app_name: Some("Lark App".to_string()),
            app_status: Some("active".to_string()),
            verification_status: Some("pending".to_string()),
            verification_type: Some("marketplace".to_string()),
            verification_time: Some("1640995200000".to_string()),
            expire_time: None,
            scopes: Some(vec![
                "im:write".to_string(),
                "contact:write".to_string(),
                "calendar:read".to_string(),
            ]),
            tenant_info: Some(TenantInfo {
                tenant_key: Some("test_tenant_key".to_string()),
                tenant_name: Some("Acme Corp".to_string()),
            }),
        };

        let json = serde_json::to_string(&verification_info).unwrap();

        assert!(json.contains("app_123"));
        assert!(json.contains("Lark App"));
        assert!(json.contains("pending"));
        assert!(json.contains("im:write"));
        assert!(json.contains("test_tenant_key"));
        assert!(json.contains("Acme Corp"));
        assert!(!json.contains("expire_time"));

        let deserialized: VerificationInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_id, Some("app_123".to_string()));
        assert_eq!(
            deserialized.verification_status,
            Some("pending".to_string())
        );
        assert_eq!(deserialized.expire_time, None);
        assert_eq!(deserialized.scopes.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_verification_info_with_none_values() {
        let verification_info = VerificationInfo {
            app_id: Some("minimal_app".to_string()),
            app_name: None,
            app_status: None,
            verification_status: None,
            verification_type: None,
            verification_time: None,
            expire_time: None,
            scopes: None,
            tenant_info: None,
        };

        let json = serde_json::to_string(&verification_info).unwrap();

        assert!(json.contains("minimal_app"));
        assert!(!json.contains("app_name"));
        assert!(!json.contains("app_status"));
        assert!(!json.contains("verification_status"));
        assert!(!json.contains("scopes"));
        assert!(!json.contains("tenant_info"));
    }

    #[test]
    fn test_verification_info_debug() {
        let verification_info = VerificationInfo {
            app_id: Some("debug_test".to_string()),
            app_name: Some("Debug Test App".to_string()),
            app_status: Some("testing".to_string()),
            verification_status: Some("in_progress".to_string()),
            verification_type: Some("internal".to_string()),
            verification_time: Some("1672531200000".to_string()),
            expire_time: Some("1704067200000".to_string()),
            scopes: Some(vec!["debug:read".to_string()]),
            tenant_info: Some(TenantInfo {
                tenant_key: Some("debug_tenant".to_string()),
                tenant_name: Some("Debug Org".to_string()),
            }),
        };

        let debug_output = format!("{:?}", verification_info);
        assert!(debug_output.contains("VerificationInfo"));
        assert!(debug_output.contains("debug_test"));
        assert!(debug_output.contains("Debug Test App"));
        assert!(debug_output.contains("in_progress"));
    }

    #[test]
    fn test_tenant_info_creation() {
        let tenant_info = TenantInfo {
            tenant_key: Some("tenant_456".to_string()),
            tenant_name: Some("Sample Tenant".to_string()),
        };

        assert_eq!(tenant_info.tenant_key, Some("tenant_456".to_string()));
        assert_eq!(tenant_info.tenant_name, Some("Sample Tenant".to_string()));
    }

    #[test]
    fn test_tenant_info_serialization() {
        let tenant_info = TenantInfo {
            tenant_key: Some("serialization_test".to_string()),
            tenant_name: Some("Serialization Test Org".to_string()),
        };

        let json = serde_json::to_string(&tenant_info).unwrap();
        assert!(json.contains("serialization_test"));
        assert!(json.contains("Serialization Test Org"));

        let deserialized: TenantInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.tenant_key,
            Some("serialization_test".to_string())
        );
        assert_eq!(
            deserialized.tenant_name,
            Some("Serialization Test Org".to_string())
        );
    }

    #[test]
    fn test_tenant_info_with_none_values() {
        let tenant_info = TenantInfo {
            tenant_key: None,
            tenant_name: None,
        };

        let json = serde_json::to_string(&tenant_info).unwrap();
        assert_eq!(json, "{}");

        let deserialized: TenantInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tenant_key, None);
        assert_eq!(deserialized.tenant_name, None);
    }

    #[test]
    fn test_tenant_info_partial_data() {
        let tenant_info_key_only = TenantInfo {
            tenant_key: Some("key_only".to_string()),
            tenant_name: None,
        };

        let json = serde_json::to_string(&tenant_info_key_only).unwrap();
        assert!(json.contains("key_only"));
        assert!(!json.contains("tenant_name"));

        let tenant_info_name_only = TenantInfo {
            tenant_key: None,
            tenant_name: Some("Name Only Org".to_string()),
        };

        let json2 = serde_json::to_string(&tenant_info_name_only).unwrap();
        assert!(json2.contains("Name Only Org"));
        assert!(!json2.contains("tenant_key"));
    }

    #[test]
    fn test_tenant_info_debug() {
        let tenant_info = TenantInfo {
            tenant_key: Some("debug_key".to_string()),
            tenant_name: Some("Debug Tenant".to_string()),
        };

        let debug_output = format!("{:?}", tenant_info);
        assert!(debug_output.contains("TenantInfo"));
        assert!(debug_output.contains("debug_key"));
        assert!(debug_output.contains("Debug Tenant"));
    }

    #[test]
    fn test_verification_info_scopes_edge_cases() {
        let empty_scopes = VerificationInfo {
            app_id: Some("empty_scopes_test".to_string()),
            app_name: Some("Empty Scopes Test".to_string()),
            app_status: Some("active".to_string()),
            verification_status: Some("verified".to_string()),
            verification_type: Some("test".to_string()),
            verification_time: Some("1600000000000".to_string()),
            expire_time: Some("1700000000000".to_string()),
            scopes: Some(vec![]),
            tenant_info: None,
        };

        let json = serde_json::to_string(&empty_scopes).unwrap();
        assert!(json.contains("\"scopes\":[]"));

        let single_scope = VerificationInfo {
            app_id: Some("single_scope_test".to_string()),
            app_name: Some("Single Scope Test".to_string()),
            app_status: Some("active".to_string()),
            verification_status: Some("verified".to_string()),
            verification_type: Some("test".to_string()),
            verification_time: Some("1600000000000".to_string()),
            expire_time: Some("1700000000000".to_string()),
            scopes: Some(vec!["single:scope".to_string()]),
            tenant_info: None,
        };

        let json2 = serde_json::to_string(&single_scope).unwrap();
        assert!(json2.contains("single:scope"));
        assert_eq!(single_scope.scopes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_verification_info_timestamp_handling() {
        let info_with_timestamps = VerificationInfo {
            app_id: Some("timestamp_test".to_string()),
            app_name: Some("Timestamp Test".to_string()),
            app_status: Some("active".to_string()),
            verification_status: Some("verified".to_string()),
            verification_type: Some("automated".to_string()),
            verification_time: Some("1640995200000".to_string()),
            expire_time: Some("1672531200000".to_string()),
            scopes: Some(vec!["time:test".to_string()]),
            tenant_info: None,
        };

        assert!(info_with_timestamps
            .verification_time
            .as_ref()
            .unwrap()
            .parse::<u64>()
            .is_ok());
        assert!(info_with_timestamps
            .expire_time
            .as_ref()
            .unwrap()
            .parse::<u64>()
            .is_ok());

        let verification_time: u64 = info_with_timestamps
            .verification_time
            .unwrap()
            .parse()
            .unwrap();
        let expire_time: u64 = info_with_timestamps.expire_time.unwrap().parse().unwrap();
        assert!(expire_time > verification_time);
    }
}
