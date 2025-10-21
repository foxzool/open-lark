use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 国家/地区信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryRegion {
    /// 主数据编码
    pub master_data_code: String,
    /// 国家/地区名称
    pub name: String,
    /// 国家/地区英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 国家/地区代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// ISO 国家代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    /// 区域类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// 排序序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    /// 描述信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 用户数据维度关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthDataRelation {
    /// 关系ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 用户ID
    pub user_id: String,
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 数据维度名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_dimension_name: Option<String>,
    /// 数据维度类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_dimension_type: Option<String>,
    /// 绑定类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_type: Option<String>,
    /// 权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end_time: Option<i64>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 数据维度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDimension {
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 数据维度名称
    pub name: String,
    /// 数据维度描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据维度类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    /// 数据维度分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page_response = PageResponse {
            items: vec![
                "item1".to_string(),
                "item2".to_string(),
                "item3".to_string(),
            ],
            page_token: Some("next_page_token_123".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items, deserialized.items);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert_eq!(page_response.items.len(), 3);
    }

    #[test]
    fn test_page_response_empty_serialization() {
        let page_response: PageResponse<String> = PageResponse {
            items: vec![],
            page_token: None,
            has_more: Some(false),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items, deserialized.items);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert!(page_response.items.is_empty());
    }

    #[test]
    fn test_page_response_with_complex_items() {
        let page_response = PageResponse {
            items: vec![
                CountryRegion {
                    master_data_code: "CN".to_string(),
                    name: "中国".to_string(),
                    name_en: Some("China".to_string()),
                    country_code: Some("CN".to_string()),
                    iso_country_code: Some("CHN".to_string()),
                    region_type: Some("country".to_string()),
                    status: Some("active".to_string()),
                    created_at: Some(1640995200000),
                    updated_at: Some(1640995200000),
                    sort_order: Some(1),
                    description: Some("People's Republic of China".to_string()),
                },
                CountryRegion {
                    master_data_code: "US".to_string(),
                    name: "美国".to_string(),
                    name_en: Some("United States".to_string()),
                    country_code: Some("US".to_string()),
                    iso_country_code: Some("USA".to_string()),
                    region_type: Some("country".to_string()),
                    status: Some("active".to_string()),
                    created_at: Some(1640995200000),
                    updated_at: Some(1640995200000),
                    sort_order: Some(2),
                    description: Some("United States of America".to_string()),
                },
            ],
            page_token: Some("country_page_2".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<CountryRegion> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items.len(), deserialized.items.len());
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert_eq!(
            page_response.items[0].master_data_code,
            deserialized.items[0].master_data_code
        );
        assert_eq!(page_response.items[1].name, deserialized.items[1].name);
    }

    #[test]
    fn test_country_region_serialization() {
        let country = CountryRegion {
            master_data_code: "JP".to_string(),
            name: "日本".to_string(),
            name_en: Some("Japan".to_string()),
            country_code: Some("JP".to_string()),
            iso_country_code: Some("JPN".to_string()),
            region_type: Some("country".to_string()),
            status: Some("active".to_string()),
            created_at: Some(1640995200000),
            updated_at: Some(1672531200000),
            sort_order: Some(5),
            description: Some("Japan Island Nation".to_string()),
        };

        let serialized = serde_json::to_string(&country).unwrap();
        let deserialized: CountryRegion = serde_json::from_str(&serialized).unwrap();

        assert_eq!(country.master_data_code, deserialized.master_data_code);
        assert_eq!(country.name, deserialized.name);
        assert_eq!(country.name_en, deserialized.name_en);
        assert_eq!(country.country_code, deserialized.country_code);
        assert_eq!(country.iso_country_code, deserialized.iso_country_code);
        assert_eq!(country.region_type, deserialized.region_type);
        assert_eq!(country.status, deserialized.status);
        assert_eq!(country.created_at, deserialized.created_at);
        assert_eq!(country.updated_at, deserialized.updated_at);
        assert_eq!(country.sort_order, deserialized.sort_order);
        assert_eq!(country.description, deserialized.description);
    }

    #[test]
    fn test_country_region_minimal_serialization() {
        let country = CountryRegion {
            master_data_code: "FR".to_string(),
            name: "法国".to_string(),
            name_en: None,
            country_code: None,
            iso_country_code: None,
            region_type: None,
            status: None,
            created_at: None,
            updated_at: None,
            sort_order: None,
            description: None,
        };

        let serialized = serde_json::to_string(&country).unwrap();
        let deserialized: CountryRegion = serde_json::from_str(&serialized).unwrap();

        assert_eq!(country.master_data_code, deserialized.master_data_code);
        assert_eq!(country.name, deserialized.name);
        assert_eq!(country.name_en, deserialized.name_en);
        assert_eq!(country.country_code, deserialized.country_code);
        assert_eq!(country.iso_country_code, deserialized.iso_country_code);
        assert_eq!(country.region_type, deserialized.region_type);
        assert_eq!(country.status, deserialized.status);
        assert_eq!(country.created_at, deserialized.created_at);
        assert_eq!(country.updated_at, deserialized.updated_at);
        assert_eq!(country.sort_order, deserialized.sort_order);
        assert_eq!(country.description, deserialized.description);
    }

    #[test]
    fn test_user_auth_data_relation_serialization() {
        let relation = UserAuthDataRelation {
            relation_id: Some("rel_123456".to_string()),
            user_id: "user_001".to_string(),
            data_dimension_id: "dim_geo_001".to_string(),
            data_dimension_name: Some("Geographic Region".to_string()),
            data_dimension_type: Some("geographic".to_string()),
            bind_type: Some("direct".to_string()),
            permission_level: Some("read_write".to_string()),
            status: Some("active".to_string()),
            effective_start_time: Some(1640995200000),
            effective_end_time: Some(1735689600000),
            creator: Some("admin_001".to_string()),
            created_at: Some(1640995200000),
            updated_at: Some(1672531200000),
        };

        let serialized = serde_json::to_string(&relation).unwrap();
        let deserialized: UserAuthDataRelation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(relation.relation_id, deserialized.relation_id);
        assert_eq!(relation.user_id, deserialized.user_id);
        assert_eq!(relation.data_dimension_id, deserialized.data_dimension_id);
        assert_eq!(
            relation.data_dimension_name,
            deserialized.data_dimension_name
        );
        assert_eq!(
            relation.data_dimension_type,
            deserialized.data_dimension_type
        );
        assert_eq!(relation.bind_type, deserialized.bind_type);
        assert_eq!(relation.permission_level, deserialized.permission_level);
        assert_eq!(relation.status, deserialized.status);
        assert_eq!(
            relation.effective_start_time,
            deserialized.effective_start_time
        );
        assert_eq!(relation.effective_end_time, deserialized.effective_end_time);
        assert_eq!(relation.creator, deserialized.creator);
        assert_eq!(relation.created_at, deserialized.created_at);
        assert_eq!(relation.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_user_auth_data_relation_minimal_serialization() {
        let relation = UserAuthDataRelation {
            relation_id: None,
            user_id: "user_002".to_string(),
            data_dimension_id: "dim_org_002".to_string(),
            data_dimension_name: None,
            data_dimension_type: None,
            bind_type: None,
            permission_level: None,
            status: None,
            effective_start_time: None,
            effective_end_time: None,
            creator: None,
            created_at: None,
            updated_at: None,
        };

        let serialized = serde_json::to_string(&relation).unwrap();
        let deserialized: UserAuthDataRelation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(relation.relation_id, deserialized.relation_id);
        assert_eq!(relation.user_id, deserialized.user_id);
        assert_eq!(relation.data_dimension_id, deserialized.data_dimension_id);
        assert_eq!(
            relation.data_dimension_name,
            deserialized.data_dimension_name
        );
        assert_eq!(
            relation.data_dimension_type,
            deserialized.data_dimension_type
        );
        assert_eq!(relation.bind_type, deserialized.bind_type);
        assert_eq!(relation.permission_level, deserialized.permission_level);
        assert_eq!(relation.status, deserialized.status);
        assert_eq!(
            relation.effective_start_time,
            deserialized.effective_start_time
        );
        assert_eq!(relation.effective_end_time, deserialized.effective_end_time);
        assert_eq!(relation.creator, deserialized.creator);
        assert_eq!(relation.created_at, deserialized.created_at);
        assert_eq!(relation.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_data_dimension_serialization() {
        let dimension = DataDimension {
            data_dimension_id: "dim_product_001".to_string(),
            name: "Product Category".to_string(),
            description: Some(
                "Product categorization dimension for data access control".to_string(),
            ),
            dimension_type: Some("hierarchical".to_string()),
            category: Some("business".to_string()),
            is_enabled: Some(true),
            created_at: Some(1640995200000),
            updated_at: Some(1672531200000),
        };

        let serialized = serde_json::to_string(&dimension).unwrap();
        let deserialized: DataDimension = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dimension.data_dimension_id, deserialized.data_dimension_id);
        assert_eq!(dimension.name, deserialized.name);
        assert_eq!(dimension.description, deserialized.description);
        assert_eq!(dimension.dimension_type, deserialized.dimension_type);
        assert_eq!(dimension.category, deserialized.category);
        assert_eq!(dimension.is_enabled, deserialized.is_enabled);
        assert_eq!(dimension.created_at, deserialized.created_at);
        assert_eq!(dimension.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_data_dimension_minimal_serialization() {
        let dimension = DataDimension {
            data_dimension_id: "dim_minimal_001".to_string(),
            name: "Minimal Dimension".to_string(),
            description: None,
            dimension_type: None,
            category: None,
            is_enabled: None,
            created_at: None,
            updated_at: None,
        };

        let serialized = serde_json::to_string(&dimension).unwrap();
        let deserialized: DataDimension = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dimension.data_dimension_id, deserialized.data_dimension_id);
        assert_eq!(dimension.name, deserialized.name);
        assert_eq!(dimension.description, deserialized.description);
        assert_eq!(dimension.dimension_type, deserialized.dimension_type);
        assert_eq!(dimension.category, deserialized.category);
        assert_eq!(dimension.is_enabled, deserialized.is_enabled);
        assert_eq!(dimension.created_at, deserialized.created_at);
        assert_eq!(dimension.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_data_dimension_disabled_serialization() {
        let dimension = DataDimension {
            data_dimension_id: "dim_disabled_001".to_string(),
            name: "Disabled Dimension".to_string(),
            description: Some("This dimension is currently disabled".to_string()),
            dimension_type: Some("flat".to_string()),
            category: Some("test".to_string()),
            is_enabled: Some(false),
            created_at: Some(1640995200000),
            updated_at: Some(1672531200000),
        };

        let serialized = serde_json::to_string(&dimension).unwrap();
        let deserialized: DataDimension = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dimension.is_enabled, Some(false));
        assert_eq!(deserialized.is_enabled, Some(false));
        assert_eq!(dimension.category, Some("test".to_string()));
        assert_eq!(dimension.dimension_type, Some("flat".to_string()));
    }

    #[test]
    fn test_skip_serializing_if_none_functionality() {
        let country = CountryRegion {
            master_data_code: "TEST".to_string(),
            name: "Test Country".to_string(),
            name_en: Some("Test".to_string()),
            country_code: None,
            iso_country_code: None,
            region_type: Some("test".to_string()),
            status: None,
            created_at: Some(1640995200000),
            updated_at: None,
            sort_order: None,
            description: None,
        };

        let json = serde_json::to_string(&country).unwrap();

        // Should not contain None fields
        assert!(!json.contains("country_code"));
        assert!(!json.contains("iso_country_code"));
        assert!(!json.contains("status"));
        assert!(!json.contains("updated_at"));
        assert!(!json.contains("sort_order"));
        assert!(!json.contains("description"));

        // Should contain fields with values
        assert!(json.contains("master_data_code"));
        assert!(json.contains("name"));
        assert!(json.contains("name_en"));
        assert!(json.contains("region_type"));
        assert!(json.contains("created_at"));
        assert!(json.contains("Test Country"));
        assert!(json.contains("TEST"));
    }

    #[test]
    fn test_page_response_skip_serializing_if_none() {
        let page_response: PageResponse<String> = PageResponse {
            items: vec!["test".to_string()],
            page_token: None,
            has_more: None,
        };

        let json = serde_json::to_string(&page_response).unwrap();

        // Should not contain None fields
        assert!(!json.contains("page_token"));
        assert!(!json.contains("has_more"));

        // Should contain fields with values
        assert!(json.contains("items"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_user_auth_data_relation_skip_serializing_if_none() {
        let relation = UserAuthDataRelation {
            relation_id: Some("rel_test".to_string()),
            user_id: "user_test".to_string(),
            data_dimension_id: "dim_test".to_string(),
            data_dimension_name: None,
            data_dimension_type: Some("test_type".to_string()),
            bind_type: None,
            permission_level: None,
            status: Some("active".to_string()),
            effective_start_time: None,
            effective_end_time: None,
            creator: None,
            created_at: Some(1640995200000),
            updated_at: None,
        };

        let json = serde_json::to_string(&relation).unwrap();

        // Should not contain None fields
        assert!(!json.contains("data_dimension_name"));
        assert!(!json.contains("bind_type"));
        assert!(!json.contains("permission_level"));
        assert!(!json.contains("effective_start_time"));
        assert!(!json.contains("effective_end_time"));
        assert!(!json.contains("creator"));
        assert!(!json.contains("updated_at"));

        // Should contain fields with values
        assert!(json.contains("relation_id"));
        assert!(json.contains("user_id"));
        assert!(json.contains("data_dimension_id"));
        assert!(json.contains("data_dimension_type"));
        assert!(json.contains("status"));
        assert!(json.contains("created_at"));
        assert!(json.contains("rel_test"));
        assert!(json.contains("user_test"));
        assert!(json.contains("test_type"));
    }

    #[test]
    fn test_complex_page_response_with_user_relations() {
        let page_response = PageResponse {
            items: vec![
                UserAuthDataRelation {
                    relation_id: Some("rel_001".to_string()),
                    user_id: "user_001".to_string(),
                    data_dimension_id: "dim_001".to_string(),
                    data_dimension_name: Some("Sales Region".to_string()),
                    data_dimension_type: Some("geographic".to_string()),
                    bind_type: Some("inherited".to_string()),
                    permission_level: Some("read".to_string()),
                    status: Some("active".to_string()),
                    effective_start_time: Some(1640995200000),
                    effective_end_time: None,
                    creator: Some("system".to_string()),
                    created_at: Some(1640995200000),
                    updated_at: Some(1672531200000),
                },
                UserAuthDataRelation {
                    relation_id: Some("rel_002".to_string()),
                    user_id: "user_002".to_string(),
                    data_dimension_id: "dim_002".to_string(),
                    data_dimension_name: Some("Product Line".to_string()),
                    data_dimension_type: Some("business".to_string()),
                    bind_type: Some("direct".to_string()),
                    permission_level: Some("read_write".to_string()),
                    status: Some("active".to_string()),
                    effective_start_time: Some(1640995200000),
                    effective_end_time: Some(1735689600000),
                    creator: Some("admin".to_string()),
                    created_at: Some(1640995200000),
                    updated_at: Some(1672531200000),
                },
            ],
            page_token: Some("relations_page_2".to_string()),
            has_more: Some(false),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<UserAuthDataRelation> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items.len(), 2);
        assert_eq!(deserialized.items.len(), 2);
        assert_eq!(
            page_response.page_token,
            Some("relations_page_2".to_string())
        );
        assert_eq!(page_response.has_more, Some(false));
        assert_eq!(deserialized.items[0].user_id, "user_001");
        assert_eq!(
            deserialized.items[1].permission_level,
            Some("read_write".to_string())
        );
    }

    #[test]
    fn test_timestamp_handling() {
        let now = 1672531200000i64; // Example timestamp
        let future = 1735689600000i64; // Future timestamp

        let relation = UserAuthDataRelation {
            relation_id: Some("time_test".to_string()),
            user_id: "user_time".to_string(),
            data_dimension_id: "dim_time".to_string(),
            data_dimension_name: None,
            data_dimension_type: None,
            bind_type: None,
            permission_level: None,
            status: None,
            effective_start_time: Some(now),
            effective_end_time: Some(future),
            creator: None,
            created_at: Some(now),
            updated_at: Some(future),
        };

        let serialized = serde_json::to_string(&relation).unwrap();
        let deserialized: UserAuthDataRelation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(relation.effective_start_time, Some(now));
        assert_eq!(relation.effective_end_time, Some(future));
        assert_eq!(relation.created_at, Some(now));
        assert_eq!(relation.updated_at, Some(future));
        assert_eq!(deserialized.effective_start_time, Some(now));
        assert_eq!(deserialized.effective_end_time, Some(future));
        assert_eq!(deserialized.created_at, Some(now));
        assert_eq!(deserialized.updated_at, Some(future));
    }
}
