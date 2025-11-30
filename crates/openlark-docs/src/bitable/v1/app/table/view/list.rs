
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出视图请求
#[derive(Clone)]
pub struct ListViewsRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 数据表的 table_id
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListViewsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/views).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn builder() -> ListViewsRequestBuilder {
        ListViewsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest,
}

impl ListViewsRequestBuilder {
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

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn build(self) -> ListViewsRequest {
        self.request
    }
}

/// 列出视图响应
#[derive(Clone)]
pub struct ListViewsResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 视图信息列表
    pub items: Vec<View>,
}

impl ApiResponseTrait for ListViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出视图
pub async fn list_views(
    request: ListViewsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListViewsResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    if let Some(page_token) = &request.page_token {
        api_req
            .query_params
            .insert(page_token.to_string(), page_token.clone());
    }

    if let Some(page_size) = &request.page_size {
        api_req
            .query_params
            .insert(page_size.to_string(), page_size.to_string());
    }

    let api_resp: openlark_core::core::StandardResponse<ListViewsResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

