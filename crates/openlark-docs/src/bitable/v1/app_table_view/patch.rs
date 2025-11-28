//! 更新视图模块
//!
//! 提供多维表格视图的更新功能，支持视图名称和属性的修改。

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
use super::AppTableViewService;

/// 更新视图请求
#[derive(Clone)]
pub struct PatchViewRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 视图的 view_id
    pub view_id: String,
    /// 视图名称
    pub view_name: Option<String>,
    /// 视图的自定义属性
    pub property: Option<serde_json::Value>,
}

impl PatchViewRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::PATCH,
                BITABLE_V1_VIEWS_UPDATE.to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            view_name: None,
            property: None,
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

    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.request.view_name = Some(view_name.into());
        self
    }

    pub fn property(mut self, property: serde_json::Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn build(self) -> PatchViewRequest {
        self.request
    }
}

#[derive(Serialize)]
struct PatchViewRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<serde_json::Value>,
}

/// 更新视图响应
#[derive(Clone)]
pub struct PatchViewResponse {
    /// 视图名称
    pub view_name: String,
    /// 视图 ID
    pub view_id: String,
    /// 视图类型
    pub view_type: String,
    /// 视图的自定义属性
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_patch_view_request_builder() {
        let request = PatchViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vewTpR1urY")
            .view_name("更新后的视图名称")
            .property(json!({
                "filter_info": {
                    "conditions": [
                        {
                            "field_id": "fldxxxxxx",
                            "operator": "is",
                            "value": "完成"
                        }
                    ]
                }
            }))
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
        assert_eq!(request.view_name, Some("更新后的视图名称".to_string()));
        assert!(request.property.is_some());
    }

    #[test]
    fn test_patch_view_request_minimal() {
        let request = PatchViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vewTpR1urY")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
        assert_eq!(request.view_name, None);
        assert_eq!(request.property, None);
    }

    #[test]
    fn test_patch_view_request_body_serialization() {
        let body = PatchViewRequestBody {
            view_name: Some("新视图名称".to_string()),
            property: Some(json!({"key": "value"})),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "view_name": "新视图名称",
            "property": {"key": "value"}
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_patch_view_request_body_minimal_serialization() {
        let body = PatchViewRequestBody {
            view_name: None,
            property: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({});

        assert_eq!(serialized, expected);
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
        assert_eq!(request.view_name, None);
        assert_eq!(request.property, None);
    }

    #[test]
    fn test_complex_property_serialization() {
        let complex_property = json!({
            "filter_info": {
                "conjunction": "and",
                "conditions": [
                    {
                        "field_id": "fldxxxxxx",
                        "operator": "is",
                        "value": ["完成", "进行中"]
                    },
                    {
                        "field_id": "fldyyyyyy",
                        "operator": "isNotEmpty"
                    }
                ]
            },
            "sort_info": [
                {
                    "field_id": "fldzzzzzz",
                    "desc": true
                }
            ]
        });

        let body = PatchViewRequestBody {
            view_name: Some("复杂视图".to_string()),
            property: Some(complex_property.clone()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "view_name": "复杂视图",
            "property": complex_property
        });

        assert_eq!(serialized, expected);
    }
}