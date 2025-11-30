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

/// 新增视图请求
#[derive(Clone)]
pub struct CreateViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 数据表的 table_id
    #[serde(skip)]
    table_id: String,
    /// 视图信息
    view: ViewData,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl CreateViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/tables/{}/views".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            view: ViewData::default(),
            user_id_type: None,
        }
    }

    pub fn builder() -> CreateViewRequestBuilder {
        CreateViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateViewRequestBuilder {
    request: CreateViewRequest,
}

impl CreateViewRequestBuilder {
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

    pub fn view(mut self, view: ViewData) -> Self {
        self.request.view = view;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> CreateViewRequest {
        self.request
    }
}

/// 视图数据
#[derive(Clone, Default)]
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型
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

    /// 创建表格视图
    pub fn grid_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("grid".to_string()),
            property: None,
        }
    }

    /// 创建看板视图
    pub fn kanban_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("kanban".to_string()),
            property: None,
        }
    }

    /// 创建画册视图
    pub fn gallery_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gallery".to_string()),
            property: None,
        }
    }

    /// 创建甘特视图
    pub fn gantt_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gantt".to_string()),
            property: None,
        }
    }

    /// 设置视图类型
    pub fn with_view_type(mut self, view_type: impl ToString) -> Self {
        self.view_type = Some(view_type.to_string());
        self
    }

    /// 设置视图属性
    pub fn with_property(mut self, property: Value) -> Self {
        self.property = Some(property);
        self
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CreateViewRequestBody {
    view: ViewData,
}

/// 创建视图响应
#[derive(Clone)]
pub struct CreateViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for CreateViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建视图
pub async fn create_view(
    request: CreateViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateViewResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_VIEWS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = CreateViewRequestBody {
        view: request.view,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CreateViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_view_request_builder() {
        let view = ViewData::grid_view("测试表格视图");
        let request = CreateViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view(view)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view.view_name, "测试表格视图");
        assert_eq!(request.view.view_type, Some("grid".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_view_data_types() {
        let grid_view = ViewData::grid_view("表格视图");
        assert_eq!(grid_view.view_type, Some("grid".to_string()));

        let kanban_view = ViewData::kanban_view("看板视图");
        assert_eq!(kanban_view.view_type, Some("kanban".to_string()));

        let gallery_view = ViewData::gallery_view("画册视图");
        assert_eq!(gallery_view.view_type, Some("gallery".to_string()));

        let gantt_view = ViewData::gantt_view("甘特视图");
        assert_eq!(gantt_view.view_type, Some("gantt".to_string()));
    }

    #[test]
    fn test_view_data_with_property() {
        let view = ViewData::new("自定义视图")
            .with_view_type("calendar")
            .with_property(json!({
                "filter_info": {
                    "conditions": []
                }
            }));

        assert_eq!(view.view_name, "自定义视图");
        assert_eq!(view.view_type, Some("calendar".to_string()));
        assert!(view.property.is_some());
    }

    #[test]
    fn test_view_data_serialization() {
        let view = ViewData::grid_view("测试视图")
            .with_property(json!({
                "field_configs": [
                    {
                        "field_id": "fld123",
                        "width": 200
                    }
                ]
            }));

        let serialized = serde_json::to_value(&view).unwrap();
        let expected = json!({
            "view_name": "测试视图",
            "view_type": "grid",
            "property": {
                "field_configs": [
                    {
                        "field_id": "fld123",
                        "width": 200
                    }
                ]
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_view_request_body_serialization() {
        let view = ViewData::kanban_view("任务看板")
            .with_property(json!({
                "group_field_id": "fld123",
                "cover_field_id": "fld456"
            }));

        let body = CreateViewRequestBody { view };
        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "view": {
                "view_name": "任务看板",
                "view_type": "kanban",
                "property": {
                    "group_field_id": "fld123",
                    "cover_field_id": "fld456"
                }
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_view_request_minimal() {
        let view = ViewData::new("最小视图");
        let request = CreateViewRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .view(view)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.view.view_name, "最小视图");
        assert!(request.view.view_type.is_none());
        assert!(request.view.property.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_create_view_request_builder_chaining() {
        let view = ViewData::gallery_view("图片画廊")
            .with_property(json!({
                "cover_field_id": "fld123",
                "image_field_id": "fld456"
            }));

        let request = CreateViewRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .view(view)
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.view.view_name, "图片画廊");
        assert_eq!(request.view.view_type, Some("gallery".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_create_view_response_trait() {
        assert_eq!(CreateViewResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_view_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateViewRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.view.view_name, "");
        assert!(request.view.view_type.is_none());
        assert!(request.view.property.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_complex_view_properties() {
        let gantt_view = ViewData::gantt_view("项目甘特图")
            .with_property(json!({
                "start_field_id": "fld123",
                "end_field_id": "fld456",
                "milestone_field_id": "fld789",
                "dependency_field_id": "fld012",
                "progress_field_id": "fld345",
                "assignee_field_id": "fld678"
            }));

        assert_eq!(gantt_view.view_name, "项目甘特图");
        assert_eq!(gantt_view.view_type, Some("gantt".to_string()));
        assert!(gantt_view.property.is_some());

        let property = gantt_view.property.unwrap();
        assert_eq!(property["start_field_id"], "fld123");
        assert_eq!(property["end_field_id"], "fld456");
    }
}