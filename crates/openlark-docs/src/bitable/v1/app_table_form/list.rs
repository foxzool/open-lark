//! 列出表单模块
//!
//! 提供多维表格表单的列表查询功能，支持分页和筛选。

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
use super::AppTableFormService;

/// 表单信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormInfo {
    /// 表单的form_id
    pub form_id: String,
    /// 表单的名称
    pub name: String,
    /// 表单的创建时间
    pub created_time: String,
    /// 表单的更新时间
    pub updated_time: String,
    /// 表单的创建者信息
    pub creator: CreatorInfo,
    /// 表单是否启用
    pub is_enabled: bool,
    /// 表单描述
    pub description: Option<String>,
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

/// 列出表单请求
#[derive(Clone)]
pub struct ListFormRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 是否只返回启用的表单
    pub only_enabled: Option<bool>,
}

impl ListFormRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                BITABLE_V1_FORMS_LIST.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
            only_enabled: None,
        }
    }

    pub fn builder() -> ListFormRequestBuilder {
        ListFormRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListFormRequestBuilder {
    request: ListFormRequest,
}

impl ListFormRequestBuilder {
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

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn only_enabled(mut self, only_enabled: bool) -> Self {
        self.request.only_enabled = Some(only_enabled);
        self
    }

    pub fn build(self) -> ListFormRequest {
        self.request
    }
}

/// 列出表单响应
#[derive(Clone)]
pub struct ListFormResponse {
    /// 表单列表
    pub items: Vec<FormInfo>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for ListFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_list_form_request_builder() {
        let request = ListFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(20)
            .only_enabled(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.only_enabled, Some(true));
    }

    #[test]
    fn test_form_info_serialization() {
        let form_info = FormInfo {
            form_id: "frmxxxxxx".to_string(),
            name: "测试表单".to_string(),
            created_time: "2023-01-01T00:00:00Z".to_string(),
            updated_time: "2023-01-01T00:00:00Z".to_string(),
            creator: CreatorInfo {
                user_id: "user_123".to_string(),
                name: "张三".to_string(),
                email: Some("zhangsan@example.com".to_string()),
            },
            is_enabled: true,
            description: Some("这是一个测试表单".to_string()),
        };

        let serialized = serde_json::to_value(&form_info).unwrap();
        let expected = json!({
            "form_id": "frmxxxxxx",
            "name": "测试表单",
            "created_time": "2023-01-01T00:00:00Z",
            "updated_time": "2023-01-01T00:00:00Z",
            "creator": {
                "user_id": "user_123",
                "name": "张三",
                "email": "zhangsan@example.com"
            },
            "is_enabled": true,
            "description": "这是一个测试表单"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_creator_info_serialization() {
        let creator = CreatorInfo {
            user_id: "user_456".to_string(),
            name: "李四".to_string(),
            email: None,
        };

        let serialized = serde_json::to_value(&creator).unwrap();
        let expected = json!({
            "user_id": "user_456",
            "name": "李四"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_list_form_request_minimal() {
        let request = ListFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.only_enabled, None);
    }

    #[test]
    fn test_list_form_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListFormRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.only_enabled, None);
    }
}