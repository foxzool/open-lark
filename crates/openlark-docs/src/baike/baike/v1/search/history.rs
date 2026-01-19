//! 获取搜索历史
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/history

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 搜索历史条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHistoryItem {
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 搜索时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// 获取搜索历史响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHistoryResp {
    /// 搜索历史列表
    #[serde(default)]
    pub items: Vec<SearchHistoryItem>,
}

impl ApiResponseTrait for SearchHistoryResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取搜索历史请求
pub struct SearchHistoryRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl SearchHistoryRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchHistoryResp> {
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
                ));
            }
        }

        let mut api_request: ApiRequest<SearchHistoryResp> =
            ApiRequest::get(&BaikeApiV1::SearchHistory.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchHistoryResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
