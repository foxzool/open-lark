//! 获取词典分类
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/classification/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::Classification;
use crate::common::api_endpoints::LingoApiV1;

/// 获取词典分类响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListClassificationResp {
    /// 分类列表
    #[serde(default)]
    pub items: Vec<Classification>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListClassificationResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词典分类请求
pub struct ListClassificationRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    repo_id: Option<String>,
}

impl ListClassificationRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            repo_id: None,
        }
    }

    /// 分页大小（默认 20，范围 1~500）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 词库ID（不传默认全员词库）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListClassificationResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListClassificationResp> {
        let mut api_request: ApiRequest<ListClassificationResp> =
            ApiRequest::get(&LingoApiV1::ClassificationList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }

        let response: Response<ListClassificationResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
