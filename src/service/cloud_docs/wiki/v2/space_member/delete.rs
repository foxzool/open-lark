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

/// 删除知识空间成员请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteSpaceMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 成员id
    #[serde(skip)]
    member_id: String,
    /// 成员类型：user
    #[serde(skip)]
    member_type: Option<String>,
}

impl DeleteSpaceMemberRequest {
    pub fn builder() -> DeleteSpaceMemberRequestBuilder {
        DeleteSpaceMemberRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString, member_id: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            member_id: member_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteSpaceMemberRequestBuilder {
    request: DeleteSpaceMemberRequest,
}

impl DeleteSpaceMemberRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 成员id
    pub fn member_id(mut self, member_id: impl ToString) -> Self {
        self.request.member_id = member_id.to_string();
        self
    }

    /// 成员类型：user
    pub fn member_type(mut self, member_type: impl ToString) -> Self {
        self.request.member_type = Some(member_type.to_string());
        self
    }

    pub fn build(mut self) -> DeleteSpaceMemberRequest {
        if let Some(member_type) = &self.request.member_type {
            self.request
                .api_request
                .query_params
                .insert("member_type", member_type.clone());
        }
        self.request
    }
}

/// 删除知识空间成员响应
#[derive(Debug, Deserialize)]
pub struct DeleteSpaceMemberResponse {
    /// 删除的成员id
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除知识空间成员
pub async fn delete_space_member(
    request: DeleteSpaceMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteSpaceMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = {
        let mut path = EndpointBuilder::replace_param(
            WIKI_V2_SPACE_MEMBER_DELETE,
            "space_id",
            &request.space_id,
        );
        path = EndpointBuilder::replace_param(&path, "member_id", &request.member_id);
        path
    };
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_space_member_request_builder() {
        let request = DeleteSpaceMemberRequest::builder()
            .space_id("spcxxxxxx")
            .member_id("ou_xxxxxx")
            .member_type("user")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.member_type, Some("user".to_string()));
    }
}
