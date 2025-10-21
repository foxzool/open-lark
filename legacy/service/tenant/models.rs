use serde::{Deserialize, Serialize};

/// 企业信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    /// 企业名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 企业显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 企业头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<TenantAvatar>,
    /// 企业 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

/// 企业头像信息
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TenantAvatar {
    /// 头像 72x72 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 头像 240x240 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 头像 640x640 像素
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 原始尺寸头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 企业席位信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantProductAssignInfo {
    /// 企业内席位总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_seat_count: Option<i32>,
    /// 已分配席位数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_seat_count: Option<i32>,
    /// 历史最大分配席位数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assigned_seat_count: Option<i32>,
    /// 购买时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<String>,
    /// 到期时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 产品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// 服务状态。可能值有：
    /// - trial: 试用
    /// - paid: 付费
    /// - expired: 已过期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_tenant_avatar_serialization() {
        let avatar = TenantAvatar {
            avatar_72: Some("https://example.com/avatar_72.png".to_string()),
            avatar_240: Some("https://example.com/avatar_240.png".to_string()),
            avatar_640: Some("https://example.com/avatar_640.png".to_string()),
            avatar_origin: Some("https://example.com/avatar_origin.png".to_string()),
        };

        let serialized = serde_json::to_string(&avatar).unwrap();
        let deserialized: TenantAvatar = serde_json::from_str(&serialized).unwrap();

        assert_eq!(avatar.avatar_72, deserialized.avatar_72);
        assert_eq!(avatar.avatar_240, deserialized.avatar_240);
        assert_eq!(avatar.avatar_640, deserialized.avatar_640);
        assert_eq!(avatar.avatar_origin, deserialized.avatar_origin);
        assert_eq!(avatar, deserialized);
    }

    #[test]
    fn test_tenant_avatar_empty_serialization() {
        let avatar = TenantAvatar {
            avatar_72: None,
            avatar_240: None,
            avatar_640: None,
            avatar_origin: None,
        };

        let serialized = serde_json::to_string(&avatar).unwrap();
        let deserialized: TenantAvatar = serde_json::from_str(&serialized).unwrap();

        assert_eq!(avatar.avatar_72, deserialized.avatar_72);
        assert_eq!(avatar.avatar_240, deserialized.avatar_240);
        assert_eq!(avatar.avatar_640, deserialized.avatar_640);
        assert_eq!(avatar.avatar_origin, deserialized.avatar_origin);
        assert_eq!(avatar, deserialized);
    }

    #[test]
    fn test_tenant_avatar_partial_serialization() {
        let avatar = TenantAvatar {
            avatar_72: Some("https://example.com/avatar_72.png".to_string()),
            avatar_240: None,
            avatar_640: Some("https://example.com/avatar_640.png".to_string()),
            avatar_origin: None,
        };

        let serialized = serde_json::to_string(&avatar).unwrap();
        let deserialized: TenantAvatar = serde_json::from_str(&serialized).unwrap();

        assert_eq!(avatar.avatar_72, deserialized.avatar_72);
        assert_eq!(avatar.avatar_240, deserialized.avatar_240);
        assert_eq!(avatar.avatar_640, deserialized.avatar_640);
        assert_eq!(avatar.avatar_origin, deserialized.avatar_origin);
        assert_eq!(avatar, deserialized);
    }

    #[test]
    fn test_tenant_serialization() {
        let tenant = Tenant {
            name: Some("Example Corp".to_string()),
            display_name: Some("Example Corporation".to_string()),
            avatar: Some(TenantAvatar {
                avatar_72: Some("https://example.com/avatar_72.png".to_string()),
                avatar_240: Some("https://example.com/avatar_240.png".to_string()),
                avatar_640: Some("https://example.com/avatar_640.png".to_string()),
                avatar_origin: Some("https://example.com/avatar_origin.png".to_string()),
            }),
            tenant_key: Some("tenant_123456".to_string()),
        };

        let serialized = serde_json::to_string(&tenant).unwrap();
        let deserialized: Tenant = serde_json::from_str(&serialized).unwrap();

        assert_eq!(tenant.name, deserialized.name);
        assert_eq!(tenant.display_name, deserialized.display_name);
        assert_eq!(tenant.tenant_key, deserialized.tenant_key);

        if let (Some(original), Some(deserialized_avatar)) = (&tenant.avatar, &deserialized.avatar)
        {
            assert_eq!(original, deserialized_avatar);
        }
    }

    #[test]
    fn test_tenant_empty_serialization() {
        let tenant = Tenant {
            name: None,
            display_name: None,
            avatar: None,
            tenant_key: None,
        };

        let serialized = serde_json::to_string(&tenant).unwrap();
        let deserialized: Tenant = serde_json::from_str(&serialized).unwrap();

        assert_eq!(tenant.name, deserialized.name);
        assert_eq!(tenant.display_name, deserialized.display_name);
        assert_eq!(tenant.avatar, deserialized.avatar);
        assert_eq!(tenant.tenant_key, deserialized.tenant_key);
    }

    #[test]
    fn test_tenant_partial_data_serialization() {
        let tenant = Tenant {
            name: Some("Partial Corp".to_string()),
            display_name: None,
            avatar: Some(TenantAvatar {
                avatar_72: Some("https://example.com/avatar_72.png".to_string()),
                avatar_240: None,
                avatar_640: None,
                avatar_origin: None,
            }),
            tenant_key: Some("tenant_partial".to_string()),
        };

        let serialized = serde_json::to_string(&tenant).unwrap();
        let deserialized: Tenant = serde_json::from_str(&serialized).unwrap();

        assert_eq!(tenant.name, deserialized.name);
        assert_eq!(tenant.display_name, deserialized.display_name);
        assert_eq!(tenant.tenant_key, deserialized.tenant_key);

        if let (Some(original), Some(deserialized_avatar)) = (&tenant.avatar, &deserialized.avatar)
        {
            assert_eq!(original, deserialized_avatar);
        }
    }

    #[test]
    fn test_tenant_product_assign_info_serialization() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(1000),
            assigned_seat_count: Some(750),
            max_assigned_seat_count: Some(800),
            purchase_time: Some("1640995200000".to_string()),
            expire_time: Some("1672531200000".to_string()),
            product_name: Some("Business Plan".to_string()),
            service_status: Some("paid".to_string()),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.total_seat_count, deserialized.total_seat_count);
        assert_eq!(info.assigned_seat_count, deserialized.assigned_seat_count);
        assert_eq!(
            info.max_assigned_seat_count,
            deserialized.max_assigned_seat_count
        );
        assert_eq!(info.purchase_time, deserialized.purchase_time);
        assert_eq!(info.expire_time, deserialized.expire_time);
        assert_eq!(info.product_name, deserialized.product_name);
        assert_eq!(info.service_status, deserialized.service_status);
    }

    #[test]
    fn test_tenant_product_assign_info_empty_serialization() {
        let info = TenantProductAssignInfo {
            total_seat_count: None,
            assigned_seat_count: None,
            max_assigned_seat_count: None,
            purchase_time: None,
            expire_time: None,
            product_name: None,
            service_status: None,
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.total_seat_count, deserialized.total_seat_count);
        assert_eq!(info.assigned_seat_count, deserialized.assigned_seat_count);
        assert_eq!(
            info.max_assigned_seat_count,
            deserialized.max_assigned_seat_count
        );
        assert_eq!(info.purchase_time, deserialized.purchase_time);
        assert_eq!(info.expire_time, deserialized.expire_time);
        assert_eq!(info.product_name, deserialized.product_name);
        assert_eq!(info.service_status, deserialized.service_status);
    }

    #[test]
    fn test_tenant_product_assign_info_trial_status() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(50),
            assigned_seat_count: Some(25),
            max_assigned_seat_count: Some(30),
            purchase_time: None,
            expire_time: Some("1672531200000".to_string()),
            product_name: Some("Trial Plan".to_string()),
            service_status: Some("trial".to_string()),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let _deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.service_status, Some("trial".to_string()));
        assert_eq!(info.product_name, Some("Trial Plan".to_string()));
        assert_eq!(info.assigned_seat_count, Some(25));
    }

    #[test]
    fn test_tenant_product_assign_info_expired_status() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(200),
            assigned_seat_count: Some(0),
            max_assigned_seat_count: Some(180),
            purchase_time: Some("1609459200000".to_string()),
            expire_time: Some("1640995200000".to_string()),
            product_name: Some("Enterprise Plan".to_string()),
            service_status: Some("expired".to_string()),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let _deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.service_status, Some("expired".to_string()));
        assert_eq!(info.assigned_seat_count, Some(0));
        assert_eq!(info.max_assigned_seat_count, Some(180));
    }

    #[test]
    fn test_tenant_product_assign_info_partial_data() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(500),
            assigned_seat_count: None,
            max_assigned_seat_count: Some(400),
            purchase_time: Some("1640995200000".to_string()),
            expire_time: None,
            product_name: None,
            service_status: Some("paid".to_string()),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.total_seat_count, deserialized.total_seat_count);
        assert_eq!(info.assigned_seat_count, deserialized.assigned_seat_count);
        assert_eq!(
            info.max_assigned_seat_count,
            deserialized.max_assigned_seat_count
        );
        assert_eq!(info.purchase_time, deserialized.purchase_time);
        assert_eq!(info.expire_time, deserialized.expire_time);
        assert_eq!(info.product_name, deserialized.product_name);
        assert_eq!(info.service_status, deserialized.service_status);
    }

    #[test]
    fn test_skip_serializing_if_none() {
        let tenant = Tenant {
            name: Some("Test Corp".to_string()),
            display_name: None,
            avatar: None,
            tenant_key: Some("test_key".to_string()),
        };

        let json = serde_json::to_string(&tenant).unwrap();

        // Should not contain fields that are None
        assert!(!json.contains("display_name"));
        assert!(!json.contains("avatar"));

        // Should contain fields that have values
        assert!(json.contains("name"));
        assert!(json.contains("tenant_key"));
        assert!(json.contains("Test Corp"));
        assert!(json.contains("test_key"));
    }

    #[test]
    fn test_avatar_skip_serializing_if_none() {
        let avatar = TenantAvatar {
            avatar_72: Some("https://example.com/72.png".to_string()),
            avatar_240: None,
            avatar_640: Some("https://example.com/640.png".to_string()),
            avatar_origin: None,
        };

        let json = serde_json::to_string(&avatar).unwrap();

        // Should not contain None fields
        assert!(!json.contains("avatar_240"));
        assert!(!json.contains("avatar_origin"));

        // Should contain fields with values
        assert!(json.contains("avatar_72"));
        assert!(json.contains("avatar_640"));
        assert!(json.contains("https://example.com/72.png"));
        assert!(json.contains("https://example.com/640.png"));
    }

    #[test]
    fn test_product_assign_info_skip_serializing_if_none() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(100),
            assigned_seat_count: None,
            max_assigned_seat_count: None,
            purchase_time: Some("1640995200000".to_string()),
            expire_time: None,
            product_name: Some("Basic Plan".to_string()),
            service_status: None,
        };

        let json = serde_json::to_string(&info).unwrap();

        // Should not contain None fields
        assert!(!json.contains("assigned_seat_count"));
        assert!(!json.contains("max_assigned_seat_count"));
        assert!(!json.contains("expire_time"));
        assert!(!json.contains("service_status"));

        // Should contain fields with values
        assert!(json.contains("total_seat_count"));
        assert!(json.contains("purchase_time"));
        assert!(json.contains("product_name"));
        assert!(json.contains("100"));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("Basic Plan"));
    }

    #[test]
    fn test_complex_tenant_with_nested_avatar() {
        let tenant = Tenant {
            name: Some("Complex Corporation".to_string()),
            display_name: Some("Complex Corp Ltd.".to_string()),
            avatar: Some(TenantAvatar {
                avatar_72: Some("https://cdn.example.com/avatars/tenant_72x72.jpg".to_string()),
                avatar_240: Some("https://cdn.example.com/avatars/tenant_240x240.jpg".to_string()),
                avatar_640: None,
                avatar_origin: Some(
                    "https://cdn.example.com/avatars/tenant_original.jpg".to_string(),
                ),
            }),
            tenant_key: Some("complex_corp_2023".to_string()),
        };

        let serialized = serde_json::to_string(&tenant).unwrap();
        let deserialized: Tenant = serde_json::from_str(&serialized).unwrap();

        assert_eq!(tenant.name, deserialized.name);
        assert_eq!(tenant.display_name, deserialized.display_name);
        assert_eq!(tenant.tenant_key, deserialized.tenant_key);

        // Test nested avatar structure
        assert!(tenant.avatar.is_some());
        assert!(deserialized.avatar.is_some());

        if let (Some(original_avatar), Some(deserialized_avatar)) =
            (&tenant.avatar, &deserialized.avatar)
        {
            assert_eq!(original_avatar.avatar_72, deserialized_avatar.avatar_72);
            assert_eq!(original_avatar.avatar_240, deserialized_avatar.avatar_240);
            assert_eq!(original_avatar.avatar_640, deserialized_avatar.avatar_640);
            assert_eq!(
                original_avatar.avatar_origin,
                deserialized_avatar.avatar_origin
            );
        }
    }

    #[test]
    fn test_tenant_avatar_equality() {
        let avatar1 = TenantAvatar {
            avatar_72: Some("url1".to_string()),
            avatar_240: Some("url2".to_string()),
            avatar_640: None,
            avatar_origin: Some("url3".to_string()),
        };

        let avatar2 = TenantAvatar {
            avatar_72: Some("url1".to_string()),
            avatar_240: Some("url2".to_string()),
            avatar_640: None,
            avatar_origin: Some("url3".to_string()),
        };

        let avatar3 = TenantAvatar {
            avatar_72: Some("url1".to_string()),
            avatar_240: Some("different_url".to_string()),
            avatar_640: None,
            avatar_origin: Some("url3".to_string()),
        };

        assert_eq!(avatar1, avatar2);
        assert_ne!(avatar1, avatar3);
    }

    #[test]
    fn test_large_seat_count_values() {
        let info = TenantProductAssignInfo {
            total_seat_count: Some(999999),
            assigned_seat_count: Some(888888),
            max_assigned_seat_count: Some(900000),
            purchase_time: Some("1640995200000".to_string()),
            expire_time: Some("1735689600000".to_string()),
            product_name: Some("Enterprise Plus".to_string()),
            service_status: Some("paid".to_string()),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: TenantProductAssignInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.total_seat_count, Some(999999));
        assert_eq!(info.assigned_seat_count, Some(888888));
        assert_eq!(info.max_assigned_seat_count, Some(900000));
        assert_eq!(deserialized.total_seat_count, info.total_seat_count);
        assert_eq!(deserialized.assigned_seat_count, info.assigned_seat_count);
        assert_eq!(
            deserialized.max_assigned_seat_count,
            info.max_assigned_seat_count
        );
    }
}
