
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, HttpMethod, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 视图信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// 视图ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图配置
    pub property: Option<Value>,
    /// 创建时间
    pub created_time: String,
    /// 更新时间
    pub modified_time: String,
}

/// 更新视图请求
#[derive(Debug, Clone)]pub struct PatchViewRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 视图信息
    view: ViewData,
}

impl Default for PatchViewRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest {
                method: HttpMethod::Patch,
                url: "".to_string(),
                headers: {
                    let mut h = std::collections::HashMap::new();
                    h.insert("Content-Type".to_string(), "application/json".to_string());
                    h
                },
                query: std::collections::HashMap::new(),
                body: None,
                timeout: None,
                _phantom: std::marker::PhantomData,
            },
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
            view: ViewData::default(),
        }
    }
}

impl PatchViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest {
                method: HttpMethod::Patch,
                url: "".to_string(),
                headers: {
                    let mut h = std::collections::HashMap::new();
                    h.insert("Content-Type".to_string(), "application/json".to_string());
                    h
                },
                query: std::collections::HashMap::new(),
                body: None,
                timeout: None,
                _phantom: std::marker::PhantomData,
            },
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: Option<String>,
    /// 视图的自定义属性
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // 构建API路径
    let path = format!(
        "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
        &request.app_token, &request.table_id, &request.view_id
    );
    // 暂时不需要修改api_path，在最终发送请求时处理

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    // 设置请求体
    let body = PatchViewRequestBody {
        view: request.view,
    };

    api_req = api_req.body(openlark_core::api::RequestData::Json(serde_json::to_value(&body)?));

    let api_resp: Response<PatchViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

