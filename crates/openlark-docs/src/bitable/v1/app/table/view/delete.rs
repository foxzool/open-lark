
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除视图请求
#[derive(Clone)]
pub struct DeleteViewRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
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
}

impl DeleteViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/views/{}).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteViewRequestBuilder {
        DeleteViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteViewRequestBuilder {
    request: DeleteViewRequest,
}

impl DeleteViewRequestBuilder {
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

    pub fn build(self) -> DeleteViewRequest {
        self.request
    }
}

/// 删除视图响应
#[derive(Clone)]
pub struct DeleteViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for DeleteViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除视图
pub async fn delete_view(
    request: DeleteViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteViewResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id)
        let api_request = api_request.api_path(format!(        .replace({view_id}, &request.view_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<DeleteViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

