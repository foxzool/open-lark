use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::okr::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::okr::models::{PageResponse, Review},
};

/// OKR 复盘管理服务
pub struct ReviewService {
    pub config: Config,
}

/// 复盘查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewQueryResponse {
    /// 复盘列表
    #[serde(flatten)]
    pub reviews: PageResponse<Review>,
}

impl ApiResponseTrait for ReviewQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ReviewService {
    /// 创建 OKR 复盘管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询复盘信息
    ///
    /// 查询指定条件下的 OKR 复盘信息，支持按周期、用户等维度筛选。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回复盘信息列表
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::okr::review::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = ReviewQueryRequest {
    ///         period_id: Some("period_123".to_string()),
    ///         user_id: Some("user_456".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.okr.review.query_reviews(request, None).await?;
    ///     if let Some(data) = response.data {
    ///         for review in data.reviews.items {
    ///             println!("复盘ID: {}, 评分: {:?}", review.review_id, review.score);
    ///             if let Some(content) = review.content {
    ///                 println!("复盘内容: {}", content);
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_reviews(
        &self,
        request: ReviewQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReviewQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: OKR_V1_REVIEWS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(period_id) = request.period_id {
            api_req.query_params.insert("period_id", period_id);
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id", user_id);
        }

        if let Some(okr_id) = request.okr_id {
            api_req.query_params.insert("okr_id", okr_id);
        }

        if let Some(min_score) = request.min_score {
            api_req
                .query_params
                .insert("min_score", min_score.to_string());
        }

        if let Some(max_score) = request.max_score {
            api_req
                .query_params
                .insert("max_score", max_score.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 复盘查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReviewQueryRequest {
    /// 周期ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// OKR ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr_id: Option<String>,
    /// 最低评分筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    /// 最高评分筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
