//! 检索单条记录模块
//!
//! 提供多维表格单条记录的检索功能，根据record_id获取现有记录。

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::AppTableRecordService;

/// 检索单条记录请求
#[derive(Clone)]
pub struct GetRecordRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 记录的 record_id
    pub record_id: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 视图的唯一标识符
    pub view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    pub field_names: Option<Vec<String>>,
    /// 控制是否返回自动计算的字段
    pub automatic: Option<bool>,
}

impl GetRecordRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                BITABLE_V1_RECORDS_GET.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            view_id: None,
            field_names: None,
            automatic: None,
        }
    }

    pub fn builder() -> GetRecordRequestBuilder {
        GetRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetRecordRequestBuilder {
    request: GetRecordRequest,
}

impl GetRecordRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.request.record_id = record_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.request.view_id = Some(view_id.into());
        self
    }

    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(self) -> GetRecordRequest {
        self.request
    }
}

/// 检索单条记录响应
#[derive(Clone)]
pub struct GetRecordResponse {
    /// 记录信息
    pub record: RecordInfo,
}

/// 记录信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordInfo {
    /// 记录的record_id
    pub record_id: String,
    /// 记录的字段信息
    pub fields: serde_json::Value,
    /// 记录的创建时间
    pub created_time: String,
    /// 记录的更新时间
    pub last_modified_time: String,
    /// 记录的创建者信息
    pub created_by: Option<CreatorInfo>,
    /// 记录的最后修改者信息
    pub last_modified_by: Option<UpdaterInfo>,
}

/// 创建者信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 更新者信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdaterInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_record_request_builder() {
        let request = GetRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("recxxxxxx")
            .view_id("vewxxxxxx")
            .field_names(vec!["name".to_string(), "age".to_string()])
            .automatic(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "recxxxxxx");
        assert_eq!(request.view_id, Some("vewxxxxxx".to_string()));
        assert_eq!(request.field_names, Some(vec!["name".to_string(), "age".to_string()]));
        assert_eq!(request.automatic, Some(true));
    }

    #[test]
    fn test_get_record_request_minimal() {
        let request = GetRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("recxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "recxxxxxx");
        assert_eq!(request.view_id, None);
        assert_eq!(request.field_names, None);
        assert_eq!(request.automatic, None);
    }

    #[test]
    fn test_record_info_serialization() {
        let record_info = RecordInfo {
            record_id: "recxxxxxx".to_string(),
            fields: json!({
                "name": "张三",
                "age": 25,
                "email": "zhangsan@example.com"
            }),
            created_time: "2023-01-01T00:00:00Z".to_string(),
            last_modified_time: "2023-01-01T00:00:00Z".to_string(),
            created_by: Some(CreatorInfo {
                user_id: "user_123".to_string(),
                name: "李四".to_string(),
                email: Some("lisi@example.com".to_string()),
            }),
            last_modified_by: Some(UpdaterInfo {
                user_id: "user_456".to_string(),
                name: "王五".to_string(),
                email: None,
            }),
        };

        let serialized = serde_json::to_value(&record_info).unwrap();
        let expected = json!({
            "record_id": "recxxxxxx",
            "fields": {
                "name": "张三",
                "age": 25,
                "email": "zhangsan@example.com"
            },
            "created_time": "2023-01-01T00:00:00Z",
            "last_modified_time": "2023-01-01T00:00:00Z",
            "created_by": {
                "user_id": "user_123",
                "name": "李四",
                "email": "lisi@example.com"
            },
            "last_modified_by": {
                "user_id": "user_456",
                "name": "王五"
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_creator_info_serialization() {
        let creator = CreatorInfo {
            user_id: "user_789".to_string(),
            name: "赵六".to_string(),
            email: None,
        };

        let serialized = serde_json::to_value(&creator).unwrap();
        let expected = json!({
            "user_id": "user_789",
            "name": "赵六"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_updater_info_serialization() {
        let updater = UpdaterInfo {
            user_id: "user_101".to_string(),
            name: "钱七".to_string(),
            email: Some("qianqi@example.com".to_string()),
        };

        let serialized = serde_json::to_value(&updater).unwrap();
        let expected = json!({
            "user_id": "user_101",
            "name": "钱七",
            "email": "qianqi@example.com"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_get_record_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = GetRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.record_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.view_id, None);
        assert_eq!(request.field_names, None);
        assert_eq!(request.automatic, None);
    }

    #[test]
    fn test_get_record_request_builder_chaining() {
        // 测试构建器方法链式调用
        let request = GetRecordRequest::builder()
            .app_token("app_token_123")
            .table_id("table_id_456")
            .record_id("record_id_789")
            .user_id_type("open_id")
            .view_id("view_abc")
            .field_names(vec!["field1".to_string(), "field2".to_string()])
            .automatic(false)
            .build();

        assert_eq!(request.app_token, "app_token_123");
        assert_eq!(request.table_id, "table_id_456");
        assert_eq!(request.record_id, "record_id_789");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.view_id, Some("view_abc".to_string()));
        assert_eq!(request.field_names, Some(vec!["field1".to_string(), "field2".to_string()]));
        assert_eq!(request.automatic, Some(false));
    }
}