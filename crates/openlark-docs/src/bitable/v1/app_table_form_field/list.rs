//! 列出表单问题模块

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

/// 列出表单问题请求
#[derive(Clone)]
pub struct ListFormFieldsRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 应用的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表单的 form_id
    pub form_id: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListFormFieldsRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                BITABLE_V1_FORM_QUESTION_LIST.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn builder() -> ListFormFieldsRequestBuilder {
        ListFormFieldsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListFormFieldsRequestBuilder {
    request: ListFormFieldsRequest,
}

impl ListFormFieldsRequestBuilder {
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

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn build(self) -> ListFormFieldsRequest {
        self.request
    }
}

/// 表单问题信息
#[derive(Clone, Serialize, Deserialize)]
pub struct FormFieldInfo {
    /// 问题的ID
    pub question_id: String,
    /// 问题的标题
    pub title: String,
    /// 问题的类型
    pub question_type: String,
    /// 问题的描述
    pub description: Option<String>,
    /// 是否必填
    pub required: Option<bool>,
    /// 问题的选项（对于选择题）
    pub options: Option<Vec<String>>,
    /// 默认值
    pub default_value: Option<serde_json::Value>,
    /// 验证规则
    pub validation: Option<serde_json::Value>,
    /// 排序顺序
    pub order: Option<i32>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 列出表单问题响应
#[derive(Clone)]
pub struct ListFormFieldsResponse {
    /// 问题列表
    pub items: Option<Vec<FormFieldInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 总数量
    pub total: Option<i32>,
}

impl ApiResponseTrait for ListFormFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_form_fields_request_builder() {
        let request = ListFormFieldsRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .table_id("tblxxxxxxxxxxxxxxx")
            .form_id("formxxxxxxxxxxxxxxx")
            .user_id_type("user_id")
            .page_size(50)
            .page_token("page_token")
            .build();

        assert_eq!(request.app_token, "bascnxxxxxxxxxxxxxxx");
        assert_eq!(request.table_id, "tblxxxxxxxxxxxxxxx");
        assert_eq!(request.form_id, "formxxxxxxxxxxxxxxx");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token".to_string()));
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListFormFieldsRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .table_id("tblxxxxxxxxxxxxxxx")
            .form_id("formxxxxxxxxxxxxxxx")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_form_field_info_serialization() {
        let field = FormFieldInfo {
            question_id: "qxxxxxxxxxxxxxxx".to_string(),
            title: "姓名".to_string(),
            question_type: "text".to_string(),
            description: Some("请输入您的真实姓名".to_string()),
            required: Some(true),
            options: None,
            default_value: None,
            validation: Some(serde_json::json!({
                "min_length": 1,
                "max_length": 50
            })),
            order: Some(1),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&field).unwrap();
        let expected = serde_json::json!({
            "question_id": "qxxxxxxxxxxxxxxx",
            "title": "姓名",
            "question_type": "text",
            "description": "请输入您的真实姓名",
            "required": true,
            "validation": {
                "min_length": 1,
                "max_length": 50
            },
            "order": 1,
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_optional_fields_serialization() {
        // 测试可选字段
        let field = FormFieldInfo {
            question_id: "qxxxxxxxxxxxxxxx".to_string(),
            title: "年龄".to_string(),
            question_type: "number".to_string(),
            description: None,
            required: Some(false),
            options: None,
            default_value: Some(serde_json::json!(18)),
            validation: None,
            order: Some(2),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&field).unwrap();
        let expected = serde_json::json!({
            "question_id": "qxxxxxxxxxxxxxxx",
            "title": "年龄",
            "question_type": "number",
            "required": false,
            "default_value": 18,
            "order": 2,
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }
}