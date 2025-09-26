use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 创建知识空间请求
#[derive(Debug, Serialize, Default)]
pub struct CreateSpaceRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间名称
    name: String,
    /// 知识空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl CreateSpaceRequest {
    pub fn builder() -> CreateSpaceRequestBuilder {
        CreateSpaceRequestBuilder::default()
    }

    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateSpaceRequestBuilder {
    request: CreateSpaceRequest,
}

impl CreateSpaceRequestBuilder {
    /// 知识空间名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 知识空间描述
    pub fn description(mut self, description: impl ToString) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    pub fn build(mut self) -> CreateSpaceRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateSpaceRequestBuilder,
    crate::service::cloud_docs::wiki::v2::space::SpaceService,
    CreateSpaceRequest,
    BaseResponse<CreateSpaceResponse>,
    create
);

/// 创建的知识空间信息
#[derive(Debug, Deserialize)]
pub struct CreatedSpace {
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

/// 创建知识空间响应
#[derive(Debug, Deserialize)]
pub struct CreateSpaceResponse {
    /// 创建的知识空间信息
    pub space: CreatedSpace,
}

impl ApiResponseTrait for CreateSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间
pub async fn create_space(
    request: CreateSpaceRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSpaceResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
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
    fn test_create_space_request_builder() {
        let request = CreateSpaceRequest::builder()
            .name("我的知识空间")
            .description("这是一个测试知识空间")
            .build();

        assert_eq!(request.name, "我的知识空间");
        assert_eq!(
            request.description,
            Some("这是一个测试知识空间".to_string())
        );
    }
}
