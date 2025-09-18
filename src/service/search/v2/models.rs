use serde::{Deserialize, Serialize};

/// 搜索消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMessageRequest {
    /// 搜索关键字
    pub query: String,
    /// 分页大小，默认20，最大200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索应用请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAppRequest {
    /// 搜索关键字
    pub query: String,
    /// 分页大小，默认20，最大200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索结果项
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultItem {
    /// 结果ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 搜索响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    /// 搜索结果列表
    pub items: Vec<SearchResultItem>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据源信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// 数据源ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 数据源名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据源请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataSourceRequest {
    /// 数据源名称
    pub name: String,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// 更新数据源请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceRequest {
    /// 数据源名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// 数据源列表请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDataSourceRequest {
    /// 分页大小，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据源列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDataSourceResponse {
    /// 数据源列表
    pub items: Vec<DataSource>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据项
#[derive(Debug, Serialize, Deserialize)]
pub struct DataItem {
    /// 数据项ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 数据项属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataItemRequest {
    /// 数据项ID
    pub id: String,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 数据项属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// 批量创建数据项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateDataItemRequest {
    /// 数据项列表
    pub items: Vec<CreateDataItemRequest>,
}

/// 数据范式
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    /// 范式ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 范式名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建数据范式请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchemaRequest {
    /// 范式名称
    pub name: String,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    pub definition: serde_json::Value,
}

/// 更新数据范式请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSchemaRequest {
    /// 范式名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 范式描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 范式定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_search_message_request() {
        let request = SearchMessageRequest {
            query: "技术文档".to_string(),
            page_size: Some(50),
            page_token: Some("next_page_123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("技术文档"));
        assert!(json.contains("\"page_size\":50"));
        assert!(json.contains("next_page_123"));
    }

    #[test]
    fn test_search_message_request_minimal() {
        let request = SearchMessageRequest {
            query: "简单搜索".to_string(),
            page_size: None,
            page_token: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("简单搜索"));
        assert!(!json.contains("page_size"));
        assert!(!json.contains("page_token"));
    }

    #[test]
    fn test_search_app_request() {
        let request = SearchAppRequest {
            query: "项目管理".to_string(),
            page_size: Some(20),
            page_token: Some("app_token_456".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("项目管理"));
        assert!(json.contains("\"page_size\":20"));
        assert!(json.contains("app_token_456"));
    }

    #[test]
    fn test_search_result_item() {
        let item = SearchResultItem {
            id: Some("result_123".to_string()),
            title: Some("技术文档：API接口说明".to_string()),
            content: Some("这是关于API接口的详细说明文档...".to_string()),
            url: Some("https://docs.example.com/api".to_string()),
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T15:30:00Z".to_string()),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("result_123"));
        assert!(json.contains("技术文档：API接口说明"));
        assert!(json.contains("这是关于API接口的详细说明文档"));
        assert!(json.contains("https://docs.example.com/api"));
        assert!(json.contains("2024-01-01T10:00:00Z"));
        assert!(json.contains("2024-01-01T15:30:00Z"));
    }

    #[test]
    fn test_search_result_item_minimal() {
        let item = SearchResultItem {
            id: Some("minimal_result".to_string()),
            title: None,
            content: None,
            url: None,
            create_time: None,
            update_time: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("minimal_result"));
        assert!(!json.contains("title"));
        assert!(!json.contains("content"));
        assert!(!json.contains("url"));
    }

    #[test]
    fn test_search_response() {
        let item1 = SearchResultItem {
            id: Some("item_1".to_string()),
            title: Some("第一个结果".to_string()),
            content: Some("第一个搜索结果的内容".to_string()),
            url: Some("https://example.com/1".to_string()),
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: None,
        };

        let item2 = SearchResultItem {
            id: Some("item_2".to_string()),
            title: Some("第二个结果".to_string()),
            content: Some("第二个搜索结果的内容".to_string()),
            url: Some("https://example.com/2".to_string()),
            create_time: Some("2024-01-01T11:00:00Z".to_string()),
            update_time: None,
        };

        let response = SearchResponse {
            items: vec![item1, item2],
            has_more: Some(true),
            page_token: Some("next_token_789".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("item_1"));
        assert!(json.contains("item_2"));
        assert!(json.contains("第一个结果"));
        assert!(json.contains("第二个结果"));
        assert!(json.contains("\"has_more\":true"));
        assert!(json.contains("next_token_789"));
    }

    #[test]
    fn test_data_source() {
        let data_source = DataSource {
            id: Some("datasource_123".to_string()),
            name: Some("技术文档库".to_string()),
            description: Some("存储所有技术文档的数据源".to_string()),
            status: Some("active".to_string()),
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
            update_time: Some("2024-01-02T10:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&data_source).unwrap();
        assert!(json.contains("datasource_123"));
        assert!(json.contains("技术文档库"));
        assert!(json.contains("存储所有技术文档的数据源"));
        assert!(json.contains("\"active\""));
        assert!(json.contains("2024-01-01T09:00:00Z"));
        assert!(json.contains("2024-01-02T10:00:00Z"));
    }

    #[test]
    fn test_create_data_source_request() {
        let config = serde_json::json!({
            "connector_type": "api",
            "endpoint": "https://api.example.com",
            "auth_type": "bearer"
        });

        let request = CreateDataSourceRequest {
            name: "新数据源".to_string(),
            description: Some("这是一个新创建的数据源".to_string()),
            config: Some(config),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("新数据源"));
        assert!(json.contains("这是一个新创建的数据源"));
        assert!(json.contains("connector_type"));
        assert!(json.contains("api"));
        assert!(json.contains("https://api.example.com"));
        assert!(json.contains("bearer"));
    }

    #[test]
    fn test_update_data_source_request() {
        let config = serde_json::json!({
            "refresh_interval": 3600,
            "enabled": true
        });

        let request = UpdateDataSourceRequest {
            name: Some("更新后的数据源名称".to_string()),
            description: Some("更新后的描述".to_string()),
            config: Some(config),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("更新后的数据源名称"));
        assert!(json.contains("更新后的描述"));
        assert!(json.contains("\"refresh_interval\":3600"));
        assert!(json.contains("\"enabled\":true"));
    }

    #[test]
    fn test_list_data_source_request() {
        let request = ListDataSourceRequest {
            page_size: Some(30),
            page_token: Some("list_token_abc".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"page_size\":30"));
        assert!(json.contains("list_token_abc"));
    }

    #[test]
    fn test_list_data_source_response() {
        let ds1 = DataSource {
            id: Some("ds_1".to_string()),
            name: Some("数据源1".to_string()),
            description: Some("第一个数据源".to_string()),
            status: Some("active".to_string()),
            create_time: None,
            update_time: None,
        };

        let ds2 = DataSource {
            id: Some("ds_2".to_string()),
            name: Some("数据源2".to_string()),
            description: Some("第二个数据源".to_string()),
            status: Some("inactive".to_string()),
            create_time: None,
            update_time: None,
        };

        let response = ListDataSourceResponse {
            items: vec![ds1, ds2],
            has_more: Some(false),
            page_token: Some("end_token".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ds_1"));
        assert!(json.contains("ds_2"));
        assert!(json.contains("数据源1"));
        assert!(json.contains("数据源2"));
        assert!(json.contains("\"has_more\":false"));
        assert!(json.contains("end_token"));
    }

    #[test]
    fn test_data_item() {
        let properties = serde_json::json!({
            "category": "documentation",
            "tags": ["api", "reference"],
            "priority": "high"
        });

        let item = DataItem {
            id: Some("data_item_456".to_string()),
            title: Some("API参考文档".to_string()),
            content: Some("详细的API接口参考文档内容...".to_string()),
            url: Some("https://docs.example.com/api-ref".to_string()),
            properties: Some(properties),
            create_time: Some("2024-01-03T08:00:00Z".to_string()),
            update_time: Some("2024-01-03T16:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("data_item_456"));
        assert!(json.contains("API参考文档"));
        assert!(json.contains("详细的API接口参考文档内容"));
        assert!(json.contains("https://docs.example.com/api-ref"));
        assert!(json.contains("documentation"));
        assert!(json.contains("api"));
        assert!(json.contains("reference"));
        assert!(json.contains("high"));
    }

    #[test]
    fn test_create_data_item_request() {
        let properties = serde_json::json!({
            "author": "技术团队",
            "version": "1.0",
            "language": "zh-CN"
        });

        let request = CreateDataItemRequest {
            id: "create_item_789".to_string(),
            title: Some("新建数据项".to_string()),
            content: Some("这是新建数据项的内容".to_string()),
            url: Some("https://example.com/new-item".to_string()),
            properties: Some(properties),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("create_item_789"));
        assert!(json.contains("新建数据项"));
        assert!(json.contains("这是新建数据项的内容"));
        assert!(json.contains("https://example.com/new-item"));
        assert!(json.contains("技术团队"));
        assert!(json.contains("1.0"));
        assert!(json.contains("zh-CN"));
    }

    #[test]
    fn test_batch_create_data_item_request() {
        let item1 = CreateDataItemRequest {
            id: "batch_item_1".to_string(),
            title: Some("批量项目1".to_string()),
            content: Some("第一个批量创建的项目".to_string()),
            url: Some("https://example.com/batch1".to_string()),
            properties: None,
        };

        let item2 = CreateDataItemRequest {
            id: "batch_item_2".to_string(),
            title: Some("批量项目2".to_string()),
            content: Some("第二个批量创建的项目".to_string()),
            url: Some("https://example.com/batch2".to_string()),
            properties: None,
        };

        let request = BatchCreateDataItemRequest {
            items: vec![item1, item2],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("batch_item_1"));
        assert!(json.contains("batch_item_2"));
        assert!(json.contains("批量项目1"));
        assert!(json.contains("批量项目2"));
        assert!(json.contains("第一个批量创建的项目"));
        assert!(json.contains("第二个批量创建的项目"));
    }

    #[test]
    fn test_schema() {
        let definition = serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string", "required": true},
                "content": {"type": "string"},
                "tags": {"type": "array", "items": {"type": "string"}}
            }
        });

        let schema = Schema {
            id: Some("schema_123".to_string()),
            name: Some("文档范式".to_string()),
            description: Some("用于文档数据的标准范式".to_string()),
            definition: Some(definition),
            create_time: Some("2024-01-01T12:00:00Z".to_string()),
            update_time: Some("2024-01-02T14:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&schema).unwrap();
        assert!(json.contains("schema_123"));
        assert!(json.contains("文档范式"));
        assert!(json.contains("用于文档数据的标准范式"));
        assert!(json.contains("\"type\":\"object\""));
        assert!(json.contains("\"required\":true"));
        assert!(json.contains("2024-01-01T12:00:00Z"));
    }

    #[test]
    fn test_create_schema_request() {
        let definition = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "description": {"type": "string"},
                "category": {"type": "string", "enum": ["doc", "data", "media"]}
            },
            "required": ["name"]
        });

        let request = CreateSchemaRequest {
            name: "新建范式".to_string(),
            description: Some("这是一个新建的数据范式".to_string()),
            definition,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("新建范式"));
        assert!(json.contains("这是一个新建的数据范式"));
        assert!(json.contains("\"type\":\"object\""));
        assert!(json.contains("\"enum\":[\"doc\",\"data\",\"media\"]"));
        assert!(json.contains("\"required\":[\"name\"]"));
    }

    #[test]
    fn test_update_schema_request() {
        let definition = serde_json::json!({
            "type": "object",
            "properties": {
                "updated_field": {"type": "string"}
            }
        });

        let request = UpdateSchemaRequest {
            name: Some("更新的范式名称".to_string()),
            description: Some("更新的范式描述".to_string()),
            definition: Some(definition),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("更新的范式名称"));
        assert!(json.contains("更新的范式描述"));
        assert!(json.contains("updated_field"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_data_source = DataSource {
            id: Some("minimal_ds".to_string()),
            name: None,
            description: None,
            status: None,
            create_time: None,
            update_time: None,
        };

        let json = serde_json::to_string(&minimal_data_source).unwrap();
        assert!(json.contains("minimal_ds"));
        assert!(!json.contains("name"));
        assert!(!json.contains("description"));

        let minimal_schema = Schema {
            id: Some("minimal_schema".to_string()),
            name: None,
            description: None,
            definition: None,
            create_time: None,
            update_time: None,
        };

        let schema_json = serde_json::to_string(&minimal_schema).unwrap();
        assert!(schema_json.contains("minimal_schema"));
        assert!(!schema_json.contains("name"));
        assert!(!schema_json.contains("definition"));
    }
}
