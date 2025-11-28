//! 删除表单模块
//!
//! 提供多维表格表单的删除功能。

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

/// 删除表单请求
#[derive(Clone)]
pub struct DeleteFormRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表单的 form_id
    pub form_id: String,
}

impl DeleteFormRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::DELETE,
                BITABLE_V1_FORMS_DELETE.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
        }
    }

    pub fn builder() -> DeleteFormRequestBuilder {
        DeleteFormRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteFormRequestBuilder {
    request: DeleteFormRequest,
}

impl DeleteFormRequestBuilder {
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

    pub fn build(self) -> DeleteFormRequest {
        self.request
    }
}

/// 删除表单响应
#[derive(Clone)]
pub struct DeleteFormResponse {
    /// 是否成功删除
    pub success: bool,
    /// 删除的表单ID
    pub form_id: String,
    /// 删除时间
    pub deleted_time: String,
}

impl ApiResponseTrait for DeleteFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_form_request_builder() {
        let request = DeleteFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .form_id("frmxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.form_id, "frmxxxxxx");
    }

    #[test]
    fn test_delete_form_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteFormRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.form_id, "");
    }

    #[test]
    fn test_delete_form_response_creation() {
        let response = DeleteFormResponse {
            success: true,
            form_id: "frmxxxxxx".to_string(),
            deleted_time: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(response.success, true);
        assert_eq!(response.form_id, "frmxxxxxx");
        assert_eq!(response.deleted_time, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_delete_form_request_serialization() {
        // 测试请求结构的基本属性
        let request = DeleteFormRequest::builder()
            .app_token("test_app_token")
            .table_id("test_table_id")
            .form_id("test_form_id")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.table_id, "test_table_id");
        assert_eq!(request.form_id, "test_form_id");
    }

    #[test]
    fn test_api_response_trait() {
        // 测试API响应特征实现
        assert_eq!(DeleteFormResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_form_request_builder_chaining() {
        // 测试构建器方法链式调用
        let request = DeleteFormRequest::builder()
            .app_token("app_token_123")
            .table_id("table_id_456")
            .form_id("form_id_789")
            .build();

        assert_eq!(request.app_token, "app_token_123");
        assert_eq!(request.table_id, "table_id_456");
        assert_eq!(request.form_id, "form_id_789");
    }

    #[test]
    fn test_delete_form_response_with_different_values() {
        // 测试不同值的响应
        let response = DeleteFormResponse {
            success: false,
            form_id: "frm_error".to_string(),
            deleted_time: "2023-12-31T23:59:59Z".to_string(),
        };

        assert_eq!(response.success, false);
        assert_eq!(response.form_id, "frm_error");
        assert_eq!(response.deleted_time, "2023-12-31T23:59:59Z");
    }
}