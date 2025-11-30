
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出字段请求
#[derive(Clone)]
pub struct ListFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 视图 ID
    #[serde(skip)]
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式
    #[serde(skip)]
    text_field_as_array: Option<bool>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl ListFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/fields).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            view_id: None,
            text_field_as_array: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }

    pub fn builder() -> ListFieldRequestBuilder {
        ListFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListFieldRequestBuilder {
    request: ListFieldRequest,
}

impl ListFieldRequestBuilder {
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
        self.request.view_id = Some(view_id.into());
        self
    }

    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request.text_field_as_array = Some(text_field_as_array);
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

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> ListFieldRequest {
        self.request
    }
}

/// 列出字段响应
#[derive(Clone)]
pub struct ListFieldResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 字段信息列表
    pub items: Vec<TableField>,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出字段
pub async fn list_field(
    request: ListFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListFieldResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

    // 设置查询参数
    if let Some(view_id) = &request.view_id {
        api_req
            .query_params
            .insert(view_id.to_string(), view_id.clone());
    }

    if let Some(text_field_as_array) = &request.text_field_as_array {
        api_req
            .query_params
            .insert(text_field_as_array, text_field_as_array.to_string());
    }

    if let Some(page_token) = &request.page_token {
        api_req
            .query_params
            .insert(page_token, page_token.clone());
    }

    if let Some(page_size) = &request.page_size {
        api_req
            .query_params
            .insert(page_size, page_size.to_string());
    }

    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<ListFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

