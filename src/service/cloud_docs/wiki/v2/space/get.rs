use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 获取知识空间信息请求
#[derive(Debug, Serialize, Default)]
pub struct GetSpaceRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
}

impl GetSpaceRequest {
    pub fn builder() -> GetSpaceRequestBuilder {
        GetSpaceRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GetSpaceRequestBuilder {
    request: GetSpaceRequest,
}

impl GetSpaceRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    pub fn build(self) -> GetSpaceRequest {
        self.request
    }
}

/// 知识空间详细信息
#[derive(Debug, Deserialize)]
pub struct SpaceInfo {
    /// 知识空间id
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    #[serde(default)]
    pub description: Option<String>,
    /// 知识空间类型：personal(个人空间)、team(团队空间)
    pub space_type: Option<String>,
    /// 知识空间可见性：private(私有)、public(公开)、partial_public(部分公开)
    pub visibility: Option<String>,
    /// 创建时间戳（秒）
    pub create_time: Option<i64>,
    /// 更新时间戳（秒）
    pub update_time: Option<i64>,
}

/// 获取知识空间信息响应
#[derive(Debug, Deserialize)]
pub struct GetSpaceResponse {
    /// 知识空间信息
    pub space: SpaceInfo,
}

impl ApiResponseTrait for GetSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间信息
pub async fn get_space(
    request: GetSpaceRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetSpaceResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path =
        EndpointBuilder::replace_param(WIKI_V2_SPACE_GET, "space_id", &request.space_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_space_request_builder() {
        let request = GetSpaceRequest::builder().space_id("spcxxxxxx").build();

        assert_eq!(request.space_id, "spcxxxxxx");
    }
}
