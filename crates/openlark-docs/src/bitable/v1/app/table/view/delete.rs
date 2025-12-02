
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, responses::Response},

    config::Config,

    http::Transport,
    req_option::RequestOption,


    SDKResult,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 删除视图请求
pub struct DeleteViewRequest {
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

impl Default for DeleteViewRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::delete("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views/{}"),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
        }
    }
}

impl DeleteViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::delete("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views/{}"),
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    let api_resp: Response<DeleteViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

