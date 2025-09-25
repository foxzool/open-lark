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
    /// 新增视图
    pub async fn create(
        &self,
        request: CreateViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateViewResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_VIEW_CREATE
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&CreateViewRequestBody { view: request.view })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 新增视图请求
#[derive(Debug, Default)]
pub struct CreateViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图信息
    view: ViewData,
}

impl CreateViewRequest {
    pub fn builder() -> CreateViewRequestBuilder {
        CreateViewRequestBuilder::default()
    }

    /// 创建新增视图请求
    pub fn new(app_token: impl ToString, table_id: impl ToString, view: ViewData) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            view,
        }
    }
}

#[derive(Default)]
pub struct CreateViewRequestBuilder {
    request: CreateViewRequest,
}

impl CreateViewRequestBuilder {
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

    /// 视图信息
    pub fn view(mut self, view: ViewData) -> Self {
        self.request.view = view;
        self
    }

    pub fn build(self) -> CreateViewRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CreateViewRequestBuilder,
    super::AppTableViewService,
    CreateViewRequest,
    BaseResponse<CreateViewResponse>,
    create
);

/// 视图数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

impl ViewData {
    /// 创建视图数据
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
    pub fn with_property(mut self, property: serde_json::Value) -> Self {
        self.property = Some(property);
        self
    }
}

#[derive(Serialize)]
struct CreateViewRequestBody {
    view: ViewData,
}

#[derive(Deserialize, Debug)]
pub struct CreateViewResponse {
    /// 视图 ID
    pub view_id: String,
}

impl ApiResponseTrait for CreateViewResponse {
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
    fn test_create_view_request() {
        let view = ViewData::grid_view("测试表格视图");
        let request = CreateViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view(view)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view.view_name, "测试表格视图");
        assert_eq!(request.view.view_type, Some("grid".to_string()));
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
            .with_view_type("grid")
            .with_property(json!({
                "filter_info": {
                    "conditions": []
                }
            }));

        assert_eq!(view.view_name, "自定义视图");
        assert_eq!(view.view_type, Some("grid".to_string()));
        assert!(view.property.is_some());
    }
}
