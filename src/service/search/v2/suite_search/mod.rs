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
    service::search::v2::models::{SearchAppRequest, SearchMessageRequest, SearchResponse},
};

/// 套件搜索服务
pub struct SuiteSearchService {
    pub config: Config,
}

/// 搜索消息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMessageResponse {
    /// 搜索结果
    #[serde(flatten)]
    pub search_result: SearchResponse,
}

impl ApiResponseTrait for SearchMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索应用响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAppResponse {
    /// 搜索结果
    #[serde(flatten)]
    pub search_result: SearchResponse,
}

impl ApiResponseTrait for SearchAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SuiteSearchService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索消息
    ///
    /// 该接口用于搜索飞书平台内的消息内容。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 支持关键字模糊搜索
    /// - 支持分页查询
    ///
    /// # 参数
    ///
    /// - `request`: 搜索消息请求参数
    /// - `option`: 可选的请求配置
    pub async fn search_message(
        &self,
        request: SearchMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchMessageResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::search::SEARCH_V2_MESSAGE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索应用
    ///
    /// 该接口用于搜索飞书平台内的应用。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 支持应用名称模糊搜索
    /// - 支持分页查询
    ///
    /// # 参数
    ///
    /// - `request`: 搜索应用请求参数
    /// - `option`: 可选的请求配置
    pub async fn search_app(
        &self,
        request: SearchAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchAppResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::search::SEARCH_V2_APP.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        Transport::request(api_req, &self.config, option).await
    }
}
