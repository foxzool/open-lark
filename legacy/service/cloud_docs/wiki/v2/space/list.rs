use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 获取知识空间列表请求
#[derive(Debug, Serialize, Default)]
pub struct ListSpaceRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 页大小
    #[serde(skip)]
    page_size: Option<i32>,
    /// 页标记，第一次请求不填，表示从头开始遍历
    #[serde(skip)]
    page_token: Option<String>,
}

impl ListSpaceRequest {
    pub fn builder() -> ListSpaceRequestBuilder {
        ListSpaceRequestBuilder::default()
    }

    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct ListSpaceRequestBuilder {
    request: ListSpaceRequest,
}

impl ListSpaceRequestBuilder {
    /// 页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    pub fn build(mut self) -> ListSpaceRequest {
        if let Some(page_size) = self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        self.request
    }
}

/// 知识空间信息
#[derive(Debug, Deserialize)]
pub struct Space {
    /// 知识空间id
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    #[serde(default)]
    pub description: Option<String>,
    /// 知识空间类型
    pub space_type: Option<String>,
    /// 知识空间可见性
    pub visibility: Option<String>,
}

/// 获取知识空间列表响应
#[derive(Debug, Deserialize)]
pub struct ListSpaceResponse {
    /// 知识空间列表
    pub items: Vec<Space>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for ListSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间列表
pub async fn list_spaces(
    request: ListSpaceRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListSpaceResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = WIKI_V2_SPACES.to_string();
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_space_request_builder() {
        let request = ListSpaceRequest::builder()
            .page_size(20)
            .page_token("page_token_123")
            .build();

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }
}
