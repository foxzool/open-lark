use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
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
    ) -> SDKResult<BaseResponse<AccessDataSearchResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/workplace/v1/workplace_access_data/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time".to_string(), start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time".to_string(), end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id".to_string(), user_id);
        }

        if let Some(department_id) = request.department_id {
            api_req
                .query_params
                .insert("department_id".to_string(), department_id);
        }

        if let Some(access_type) = request.access_type {
            api_req
                .query_params
                .insert("access_type".to_string(), access_type);
        }

        Transport::request(api_req, &self.config, option).await
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
    ) -> SDKResult<BaseResponse<CustomAccessDataSearchResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/workplace/v1/custom_workplace_access_data/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time".to_string(), start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time".to_string(), end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id".to_string(), user_id);
        }

        if let Some(custom_workplace_id) = request.custom_workplace_id {
            api_req
                .query_params
                .insert("custom_workplace_id".to_string(), custom_workplace_id);
        }

        Transport::request(api_req, &self.config, option).await
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
    ) -> SDKResult<BaseResponse<CustomWidgetAccessDataSearchResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/workplace/v1/custom_workplace_widget_access_data/search"
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time".to_string(), start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time".to_string(), end_time.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id".to_string(), user_id);
        }

        if let Some(custom_workplace_id) = request.custom_workplace_id {
            api_req
                .query_params
                .insert("custom_workplace_id".to_string(), custom_workplace_id);
        }

        if let Some(widget_id) = request.widget_id {
            api_req
                .query_params
                .insert("widget_id".to_string(), widget_id);
        }

        Transport::request(api_req, &self.config, option).await
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
