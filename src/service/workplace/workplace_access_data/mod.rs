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
        trait_system::Service,
        SDKResult,
    },
    service::workplace::models::{
        CustomWorkplaceAccessData, CustomWorkplaceWidgetAccessData, PageResponse,
        WorkplaceAccessData,
    },
};

/// 工作台访问数据服务
pub struct WorkplaceAccessDataService {
    pub config: Config,
}

impl WorkplaceAccessDataService {
    /// 创建工作台访问数据服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取工作台访问数据
    ///
    /// 获取工作台的访问数据统计信息，支持按时间范围、用户、部门等维度查询。
    ///
    /// # Arguments
    ///
    /// * `request` - 访问数据查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回工作台访问数据列表
    pub async fn search(
        &self,
        request: AccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<AccessDataSearchResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_ACCESS_DATA_SEARCH.to_string(),
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

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(department_id) = request.department_id {
            api_req
                .query_params
                .insert(QueryParams::DEPARTMENT_ID, department_id);
        }

        if let Some(access_type) = request.access_type {
            api_req
                .query_params
                .insert(QueryParams::ACCESS_TYPE, access_type);
        }

        let api_resp: BaseResponse<AccessDataSearchResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取定制工作台访问数据
    ///
    /// 获取定制工作台的访问数据统计信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 定制工作台访问数据查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回定制工作台访问数据列表
    pub async fn search_custom(
        &self,
        request: CustomAccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CustomAccessDataSearchResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH.to_string(),
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

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(custom_workplace_id) = request.custom_workplace_id {
            api_req
                .query_params
                .insert(QueryParams::CUSTOM_WORKPLACE_ID, custom_workplace_id);
        }

        let api_resp: BaseResponse<CustomAccessDataSearchResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取定制工作台小组件访问数据
    ///
    /// 获取定制工作台小组件的访问数据统计信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 定制工作台小组件访问数据查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回定制工作台小组件访问数据列表
    pub async fn search_custom_widget(
        &self,
        request: CustomWidgetAccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CustomWidgetAccessDataSearchResponse> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: WORKPLACE_WIDGET_ACCESS_DATA_SEARCH.to_string(),
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

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(custom_workplace_id) = request.custom_workplace_id {
            api_req
                .query_params
                .insert(QueryParams::CUSTOM_WORKPLACE_ID, custom_workplace_id);
        }

        if let Some(widget_id) = request.widget_id {
            api_req
                .query_params
                .insert(QueryParams::WIDGET_ID, widget_id);
        }

        let api_resp: BaseResponse<CustomWidgetAccessDataSearchResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

impl Service for WorkplaceAccessDataService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "workplace_access_data"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 工作台访问数据查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessDataSearchRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 访问类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
}

impl AccessDataSearchRequest {
    /// 创建Builder实例
    pub fn builder() -> AccessDataSearchRequestBuilder {
        AccessDataSearchRequestBuilder::default()
    }
}

/// 工作台访问数据查询请求Builder
#[derive(Default)]
pub struct AccessDataSearchRequestBuilder {
    inner: AccessDataSearchRequest,
}

impl AccessDataSearchRequestBuilder {
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

    /// 设置开始时间戳
    pub fn start_time(mut self, timestamp: i64) -> Self {
        self.inner.start_time = Some(timestamp);
        self
    }

    /// 设置结束时间戳  
    pub fn end_time(mut self, timestamp: i64) -> Self {
        self.inner.end_time = Some(timestamp);
        self
    }

    /// 设置时间范围（复合方法）
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
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

    /// 设置访问类型筛选
    pub fn access_type_filter(mut self, access_type: impl Into<String>) -> Self {
        self.inner.access_type = Some(access_type.into());
        self
    }

    /// 构建请求对象
    pub fn build(self) -> AccessDataSearchRequest {
        self.inner
    }
}

/// 工作台访问数据查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessDataSearchResponse {
    /// 工作台访问数据列表
    #[serde(flatten)]
    pub access_data: PageResponse<WorkplaceAccessData>,
}

impl ApiResponseTrait for AccessDataSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 定制工作台访问数据查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomAccessDataSearchRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
}

/// 定制工作台访问数据查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomAccessDataSearchResponse {
    /// 定制工作台访问数据列表
    #[serde(flatten)]
    pub access_data: PageResponse<CustomWorkplaceAccessData>,
}

impl ApiResponseTrait for CustomAccessDataSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 定制工作台小组件访问数据查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomWidgetAccessDataSearchRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 小组件ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
}

impl CustomWidgetAccessDataSearchRequest {
    /// 创建Builder实例
    pub fn builder() -> CustomWidgetAccessDataSearchRequestBuilder {
        CustomWidgetAccessDataSearchRequestBuilder::default()
    }
}

/// 定制工作台小组件访问数据查询请求Builder
#[derive(Default)]
pub struct CustomWidgetAccessDataSearchRequestBuilder {
    inner: CustomWidgetAccessDataSearchRequest,
}

impl CustomWidgetAccessDataSearchRequestBuilder {
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

    /// 设置开始时间戳
    pub fn start_time(mut self, timestamp: i64) -> Self {
        self.inner.start_time = Some(timestamp);
        self
    }

    /// 设置结束时间戳  
    pub fn end_time(mut self, timestamp: i64) -> Self {
        self.inner.end_time = Some(timestamp);
        self
    }

    /// 设置时间范围（复合方法）
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
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

    /// 设置定制工作台ID筛选
    pub fn custom_workplace_filter(mut self, custom_workplace_id: impl Into<String>) -> Self {
        self.inner.custom_workplace_id = Some(custom_workplace_id.into());
        self
    }

    /// 设置小组件ID筛选
    pub fn widget_filter(mut self, widget_id: impl Into<String>) -> Self {
        self.inner.widget_id = Some(widget_id.into());
        self
    }

    /// 构建请求对象
    pub fn build(self) -> CustomWidgetAccessDataSearchRequest {
        self.inner
    }
}

/// 定制工作台小组件访问数据查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomWidgetAccessDataSearchResponse {
    /// 定制工作台小组件访问数据列表
    #[serde(flatten)]
    pub access_data: PageResponse<CustomWorkplaceWidgetAccessData>,
}

impl ApiResponseTrait for CustomWidgetAccessDataSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
