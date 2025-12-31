//! 获取词典分类
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/classification/list
//! doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/classification/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::ClassificationItem;
use crate::common::api_endpoints::BaikeApiV1;

/// 获取词典分类请求
pub struct ListClassificationRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListClassificationRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 分页大小（默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页 token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn send(self) -> SDKResult<ListClassificationResponse> {
        if let Some(page_size) = self.page_size {
            if !(1..=500).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~500",
                ));
            }
        }

        let mut api_request: ApiRequest<ListClassificationResponse> =
            ApiRequest::get(&BaikeApiV1::ClassificationList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        let response: Response<ListClassificationResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 获取词典分类响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListClassificationResponse {
    #[serde(default)]
    pub items: Vec<ClassificationItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListClassificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
