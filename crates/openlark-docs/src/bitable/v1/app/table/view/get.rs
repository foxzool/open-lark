
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

/// 获取视图请求
pub struct GetViewRequest {
    config: Config,
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl Default for GetViewRequest {
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
            view_id: String::new(),
            user_id_type: None,
        }
    }
}

impl GetViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(format!("https://open.feishu.cn/open-apis/bitable/v1/apps/{{}}/tables/{{}}/views/{{}}")),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> GetViewRequestBuilder {
        GetViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetViewRequestBuilder {
    request: GetViewRequest,
}

impl GetViewRequestBuilder {
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

    pub fn build(self) -> GetViewRequest {
        self.request
    }
}

/// 获取视图响应
#[derive(Debug, Clone)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取视图
pub async fn get_view(
    request: GetViewRequest,
    _option: Option<RequestOption>,
) -> SDKResult<GetViewResponse> {
    // 构建最终的URL
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
        request.app_token, request.table_id, request.view_id
    );

    // 创建API请求
    let mut api_request: ApiRequest<GetViewResponse> = ApiRequest::get(url);

    // 添加查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // 转换为Transport::request可以接受的格式
    let request_for_transport = ApiRequest::<()>::get(api_request.build_url());

    // 发送请求
    let config = &request.config;
    let response = Transport::request(request_for_transport, config, None).await?;

    // 解析响应数据
    let view_data: View = response.data
        .and_then(|data| serde_json::from_value(data).ok())
        .ok_or_else(|| validation_error("解析视图数据失败", "响应数据格式不正确"))?;

    Ok(GetViewResponse {
        view: view_data,
    })
}

