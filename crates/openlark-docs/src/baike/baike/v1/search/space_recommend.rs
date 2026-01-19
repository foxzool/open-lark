//! 推荐空间
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/space_recommend

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 推荐空间条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendSpaceItem {
    /// 空间 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    /// 空间名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 推荐空间响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSpaceRecommendResp {
    /// 推荐空间列表
    #[serde(default)]
    pub items: Vec<RecommendSpaceItem>,
}

impl ApiResponseTrait for SearchSpaceRecommendResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 推荐空间请求
pub struct SearchSpaceRecommendRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
}

impl SearchSpaceRecommendRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchSpaceRecommendResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchSpaceRecommendResp> {
        let mut api_request: ApiRequest<SearchSpaceRecommendResp> =
            ApiRequest::get(&BaikeApiV1::SearchSpaceRecommend.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchSpaceRecommendResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
