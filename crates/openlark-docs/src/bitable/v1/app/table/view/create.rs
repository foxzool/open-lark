
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, responses::Response},

    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 新增视图请求
#[derive(Debug, Clone)]
pub struct CreateViewRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图信息
    view: ViewData,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl Default for CreateViewRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::post("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views"),
            app_token: String::new(),
            table_id: String::new(),
            view: ViewData::default(),
            user_id_type: None,
        }
    }
}

impl CreateViewRequest {
    pub fn new(config: Config) -> Self {
        Self::default()
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

#[derive(Serialize, Default, Debug, Clone)]
/// 视图数据
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图)
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    // 设置请求体
    let body = CreateViewRequestBody {
        view: request.view,
    };

    api_req = api_req.body(openlark_core::api::RequestData::Json(serde_json::to_value(&body)?));

    let api_resp: Response<CreateViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

