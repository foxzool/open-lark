
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},

    config::Config,

    http::Transport,
    req_option::RequestOption,
    error::validation_error,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 列出视图请求
#[derive(Debug, Clone)]
pub struct ListViewsRequest {
    config: Config,
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl Default for ListViewsRequest {
    fn default() -> Self {
        // 需要一个config实例，这里使用一个临时的
        let config = Config::builder()
            .app_id("")
            .app_secret("")
            .base_url("https://open.feishu.cn")
            .build();
        Self {
            config,
            api_request: ApiRequest::post("https://open.feishu.cn"),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }
}

impl ListViewsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(format!("https://open.feishu.cn/open-apis/bitable/v1/apps/{{}}/tables/{{}}/views")),
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

#[derive(Debug, Clone)]
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest,
}

impl Default for ListViewsRequestBuilder {
    fn default() -> Self {
        Self {
            request: ListViewsRequest::default(),
        }
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    _option: Option<RequestOption>,
) -> SDKResult<ListViewsResponse> {
    // 构建最终的URL
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views",
        request.app_token, request.table_id
    );

    // 创建API请求
    let mut api_request: ApiRequest<ListViewsResponse> = ApiRequest::get(url);

    // 添加查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }

    if let Some(page_size) = &request.page_size {
        api_request = api_request.query("page_size", &page_size.to_string());
    }

    // 转换为Transport::request可以接受的格式
    let request_for_transport = ApiRequest::<()>::get(api_request.build_url());

    // 发送请求
    let config = &request.config;
    let response = Transport::request(request_for_transport, config, None).await?;

    // 解析响应数据 - 直接解析为响应结构
    let api_response: ListViewsResponse = response.data
        .and_then(|data| serde_json::from_value(data).ok())
        .ok_or_else(|| validation_error("解析视图数据失败", "响应数据格式不正确"))?;

    Ok(api_response)
}

