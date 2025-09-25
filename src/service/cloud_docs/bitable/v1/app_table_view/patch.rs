use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppTableViewService;

impl AppTableViewService {
    /// 更新视图
    pub async fn patch(
        &self,
        request: PatchViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchViewResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PATCH;
        api_req.api_path = BITABLE_V1_VIEW_PATCH
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id)
            .replace("{view_id}", &request.view_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&PatchViewRequestBody {
            view_name: request.view_name,
            property: request.property,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 更新视图请求
#[derive(Debug, Default)]
pub struct PatchViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 视图名称
    view_name: Option<String>,
    /// 视图的自定义属性
    property: Option<serde_json::Value>,
}

impl PatchViewRequest {
    pub fn builder() -> PatchViewRequestBuilder {
        PatchViewRequestBuilder::default()
    }

    /// 创建更新视图请求
    pub fn new(app_token: impl ToString, table_id: impl ToString, view_id: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            view_id: view_id.to_string(),
            view_name: None,
            property: None,
        }
    }
}

#[derive(Default)]
pub struct PatchViewRequestBuilder {
    request: PatchViewRequest,
}

impl PatchViewRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的 table_id
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 视图的 view_id
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = view_id.to_string();
        self
    }

    /// 视图名称
    pub fn view_name(mut self, view_name: impl ToString) -> Self {
        self.request.view_name = Some(view_name.to_string());
        self
    }

    /// 视图的自定义属性
    pub fn property(mut self, property: serde_json::Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn build(self) -> PatchViewRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    PatchViewRequestBuilder,
    AppTableViewService,
    PatchViewRequest,
    BaseResponse<PatchViewResponse>,
    patch
);

#[derive(Serialize)]
struct PatchViewRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct PatchViewResponse {
    /// 视图名称
    pub view_name: String,
    /// 视图 ID
    pub view_id: String,
    /// 视图类型
    pub view_type: String,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_patch_view_request() {
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
    fn test_patch_view_request_new() {
        let request =
            PatchViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

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
}
