#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::View,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新视图请求
#[derive(Clone)]
pub struct PatchViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 数据表的 table_id
    #[serde(skip)]
    table_id: String,
    /// 视图的 view_id
    #[serde(skip)]
    view_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 视图信息
    view: ViewData,
}

impl PatchViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::PATCH, "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
            view: ViewData::default(),
        }
    }

    pub fn builder() -> PatchViewRequestBuilder {
        PatchViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PatchViewRequestBuilder {
    request: PatchViewRequest,
}

impl PatchViewRequestBuilder {
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

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.request.view_id = view_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn view(mut self, view: ViewData) -> Self {
        self.request.view = view;
        self
    }

    pub fn build(self) -> PatchViewRequest {
        self.request
    }
}

/// 视图数据
#[derive(Clone, Default, Serialize)]
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 视图的自定义属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
}

impl ViewData {
    pub fn new(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: None,
            property: None,
        }
    }

    pub fn with_view_type(mut self, view_type: impl ToString) -> Self {
        self.view_type = Some(view_type.to_string());
        self
    }

    pub fn with_property(mut self, property: Value) -> Self {
        self.property = Some(property);
        self
    }
}

/// 请求体结构
#[derive(Serialize)]
struct PatchViewRequestBody {
    view: ViewData,
}

/// 更新视图响应
#[derive(Clone)]
pub struct PatchViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新视图
pub async fn patch_view(
    request: PatchViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<PatchViewResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
    api_req.api_path = BITABLE_V1_VIEW_PATCH
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{view_id}", &request.view_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = PatchViewRequestBody {
        view: request.view,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<PatchViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_patch_view_request_builder() {
        let view = ViewData::new("更新的视图")
            .with_view_type("grid")
            .with_property(json!({"sort": "asc"}));

        let request = PatchViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vew123456")
            .view(view)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vew123456");
        assert_eq!(request.view.view_name, "更新的视图");
        assert_eq!(request.view.view_type, Some("grid".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_patch_view_request_minimal() {
        let view = ViewData::new("最小更新");
        let request = PatchViewRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .view_id("test-view")
            .view(view)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.view_id, "test-view");
        assert_eq!(request.view.view_name, "最小更新");
        assert!(request.view.view_type.is_none());
        assert!(request.view.property.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_patch_view_response_trait() {
        assert_eq!(PatchViewResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_patch_view_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = PatchViewRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.view_id, "");
        assert_eq!(request.view.view_name, "");
        assert!(request.view.view_type.is_none());
        assert!(request.view.property.is_none());
        assert!(request.user_id_type.is_none());
    }
}