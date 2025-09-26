use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::performance::models::{PerformanceResult, ReviewDetail},
};

/// 评估数据管理服务
pub struct ReviewDataService {
    pub config: Config,
}

impl ReviewDataService {
    /// 创建评估数据管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取绩效结果
    ///
    /// 查询被评估人的绩效结果信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 绩效结果查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回绩效结果列表
    pub async fn query_results(
        &self,
        request: ResultQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ResultQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_REVIEW_DATA_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取绩效详情数据
    ///
    /// 查询绩效评估的详细数据，包括各评估项的具体回答。
    ///
    /// # Arguments
    ///
    /// * `request` - 绩效详情查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回绩效详情数据列表
    pub async fn query_details(
        &self,
        request: DetailQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DetailQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 绩效结果查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultQueryRequest {
    /// 周期ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 项目ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// 被评估人ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewee_ids: Option<Vec<String>>,
    /// 是否只查询已开通结果的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_opened_only: Option<bool>,
}

/// 绩效结果查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultQueryResponse {
    /// 绩效结果列表
    pub performance_results: Vec<PerformanceResult>,
}

impl ApiResponseTrait for ResultQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 绩效详情查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailQueryRequest {
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewee_ids: Option<Vec<String>>,
    /// 评估人ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_ids: Option<Vec<String>>,
    /// 评估项ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
}

/// 绩效详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailQueryResponse {
    /// 绩效详情列表
    pub review_details: Vec<ReviewDetail>,
}

impl ApiResponseTrait for DetailQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Service for ReviewDataService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "review_data"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}
