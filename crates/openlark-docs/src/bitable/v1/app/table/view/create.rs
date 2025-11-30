
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 新增视图请求
#[derive(Clone)]
pub struct CreateViewRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
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
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/views).config(config)),
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
    #[serde(skip_serializing_if = Option::is_none)]
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型
    #[serde(skip_serializing_if = Option::is_none)]
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
            view_type: Some(grid.to_string()),
            property: None,
        }
    }

    /// 创建看板视图
    pub fn kanban_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some(kanban.to_string()),
            property: None,
        }
    }

    /// 创建画册视图
    pub fn gallery_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some(gallery.to_string()),
            property: None,
        }
    }

    /// 创建甘特视图
    pub fn gantt_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some(gantt.to_string()),
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
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
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

