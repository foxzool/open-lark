use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::workplace::*,
        http::Transport,
        query_params::QueryParams,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    },
    service::workplace::models::{AppRecommendRule, FavouriteApp, PageResponse, RecommendedApp},
};

/// 应用推荐服务
pub struct AppRecommendService {
    pub config: Config,
}

impl AppRecommendService {
    /// 创建应用推荐服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户自定义常用的应用
    ///
    /// 获取当前用户自定义设置的常用应用列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 常用应用查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户常用应用列表
    pub async fn get_favourite_apps(
        &self,
        request: FavouriteAppsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<FavouriteAppsResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_FAVOURITE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        let api_resp: BaseResponse<FavouriteAppsResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取管理员推荐的应用
    ///
    /// 获取管理员设置的推荐应用列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 推荐应用查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回管理员推荐应用列表
    pub async fn get_recommended_apps(
        &self,
        request: RecommendedAppsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<RecommendedAppsResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_RECOMMEND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(department_id) = request.department_id {
            api_req
                .query_params
                .insert(QueryParams::DEPARTMENT_ID, department_id);
        }

        let api_resp: BaseResponse<RecommendedAppsResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取当前设置的推荐规则列表
    ///
    /// 获取当前系统中配置的应用推荐规则列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 推荐规则查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回推荐规则列表
    pub async fn list_recommend_rules(
        &self,
        request: RecommendRulesListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<RecommendRulesListResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(rule_type) = request.rule_type {
            api_req
                .query_params
                .insert(QueryParams::RULE_TYPE, rule_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert(QueryParams::STATUS, status);
        }

        let api_resp: BaseResponse<RecommendRulesListResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 常用应用查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FavouriteAppsRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 常用应用查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FavouriteAppsResponse {
    /// 常用应用列表
    #[serde(flatten)]
    pub favourite_apps: PageResponse<FavouriteApp>,
}

impl ApiResponseTrait for FavouriteAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 推荐应用查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommendedAppsRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

impl RecommendedAppsRequest {
    /// 创建Builder实例
    pub fn builder() -> RecommendedAppsRequestBuilder {
        RecommendedAppsRequestBuilder::default()
    }
}

/// 推荐应用查询请求Builder
#[derive(Default)]
pub struct RecommendedAppsRequestBuilder {
    inner: RecommendedAppsRequest,
}

impl RecommendedAppsRequestBuilder {
    /// 设置页面令牌
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.inner.page_token = Some(token.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.inner.page_size = Some(size);
        self
    }

    /// 设置分页参数（复合方法）
    pub fn pagination(mut self, page_token: Option<String>, page_size: Option<i32>) -> Self {
        self.inner.page_token = page_token;
        self.inner.page_size = page_size;
        self
    }

    /// 设置用户ID筛选
    pub fn user_filter(mut self, user_id: impl Into<String>) -> Self {
        self.inner.user_id = Some(user_id.into());
        self
    }

    /// 设置部门ID筛选
    pub fn department_filter(mut self, department_id: impl Into<String>) -> Self {
        self.inner.department_id = Some(department_id.into());
        self
    }

    /// 构建请求对象
    pub fn build(self) -> RecommendedAppsRequest {
        self.inner
    }
}

/// 推荐应用查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedAppsResponse {
    /// 推荐应用列表
    #[serde(flatten)]
    pub recommended_apps: PageResponse<RecommendedApp>,
}

impl ApiResponseTrait for RecommendedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 推荐规则列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommendRulesListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 规则类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl RecommendRulesListRequest {
    /// 创建Builder实例
    pub fn builder() -> RecommendRulesListRequestBuilder {
        RecommendRulesListRequestBuilder::default()
    }
}

/// 推荐规则列表查询请求Builder
#[derive(Default)]
pub struct RecommendRulesListRequestBuilder {
    inner: RecommendRulesListRequest,
}

impl RecommendRulesListRequestBuilder {
    /// 设置页面令牌
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.inner.page_token = Some(token.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.inner.page_size = Some(size);
        self
    }

    /// 设置分页参数（复合方法）
    pub fn pagination(mut self, page_token: Option<String>, page_size: Option<i32>) -> Self {
        self.inner.page_token = page_token;
        self.inner.page_size = page_size;
        self
    }

    /// 设置规则类型筛选
    pub fn rule_type_filter(mut self, rule_type: impl Into<String>) -> Self {
        self.inner.rule_type = Some(rule_type.into());
        self
    }

    /// 设置规则状态筛选
    pub fn status_filter(mut self, status: impl Into<String>) -> Self {
        self.inner.status = Some(status.into());
        self
    }

    /// 构建请求对象
    pub fn build(self) -> RecommendRulesListRequest {
        self.inner
    }
}

/// 推荐规则列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendRulesListResponse {
    /// 推荐规则列表
    #[serde(flatten)]
    pub recommend_rules: PageResponse<AppRecommendRule>,
}

impl ApiResponseTrait for RecommendRulesListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
