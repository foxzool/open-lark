
use openlark_core::{
    api::{ApiRequest, RequestData, ResponseFormat, ApiResponseTrait, responses::Response},
    config::Config,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出自定义角色请求
pub struct ListAppRoleRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl ListAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn builder() -> ListAppRoleRequestBuilder {
        ListAppRoleRequestBuilder::new()
    }
}

pub struct ListAppRoleRequestBuilder {
    request: ListAppRoleRequest,
}

impl ListAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: ListAppRoleRequest::new(Config::default()),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
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

    pub fn build(self) -> ListAppRoleRequest {
        self.request
    }
}

#[derive(Serialize)]
/// 自定义角色信息
pub struct AppRole {
    /// 自定义角色的id
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

#[derive(Serialize)]
/// 数据表权限
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    rec_rule: Option<String>,
}

/// 数据表默认权限
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
}

/// 列出自定义角色响应
pub struct ListAppRoleResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 角色信息列表
    pub items: Vec<AppRole>,
}

impl ApiResponseTrait for ListAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自定义角色
pub async fn list_app_role(
    request: ListAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListAppRoleResponse> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles",
        &request.app_token
    );
    let mut api_req = ApiRequest::<()>::get(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    if let Some(page_token) = &request.page_token {
        api_req = api_req.query("page_token", page_token);
    }

    if let Some(page_size) = &request.page_size {
        api_req = api_req.query("page_size", &page_size.to_string());
    }

    let response: Response<ListAppRoleResponse> =
        Transport::request(api_req, config, option).await?;
    response.into_result()
}

