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

/// 更新知识空间节点标题请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateSpaceNodeTitleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 节点token
    #[serde(skip)]
    node_token: String,
    /// 文档标题
    title: String,
}

impl UpdateSpaceNodeTitleRequest {
    pub fn builder() -> UpdateSpaceNodeTitleRequestBuilder {
        UpdateSpaceNodeTitleRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString, node_token: impl ToString, title: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            node_token: node_token.to_string(),
            title: title.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateSpaceNodeTitleRequestBuilder {
    request: UpdateSpaceNodeTitleRequest,
}

impl UpdateSpaceNodeTitleRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 节点token
    pub fn node_token(mut self, node_token: impl ToString) -> Self {
        self.request.node_token = node_token.to_string();
        self
    }

    /// 文档标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = title.to_string();
        self
    }

    pub fn build(mut self) -> UpdateSpaceNodeTitleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新后的节点信息
#[derive(Debug, Deserialize)]
pub struct UpdatedNode {
    /// 知识空间id
    pub space_id: String,
    /// 节点token
    pub node_token: String,
    /// 文档类型
    pub obj_type: String,
    /// 父节点token
    pub parent_node_token: Option<String>,
    /// 节点类型
    pub node_type: Option<String>,
    /// 原始文档token
    pub obj_token: Option<String>,
    /// 文档标题
    pub title: Option<String>,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Deserialize)]
pub struct UpdateSpaceNodeTitleResponse {
    /// 更新后的节点信息
    pub node: UpdatedNode,
}

impl ApiResponseTrait for UpdateSpaceNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间节点标题
pub async fn update_space_node_title(
    request: UpdateSpaceNodeTitleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateSpaceNodeTitleResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = {
        let mut path = EndpointBuilder::replace_param(
            WIKI_V2_SPACE_NODE_UPDATE_TITLE,
            "space_id",
            &request.space_id,
        );
        path = EndpointBuilder::replace_param(&path, "node_token", &request.node_token);
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
    fn test_update_space_node_title_request_builder() {
        let request = UpdateSpaceNodeTitleRequest::builder()
            .space_id("spcxxxxxx")
            .node_token("wikcnxxxxxx")
            .title("新的文档标题")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(request.title, "新的文档标题");
    }
}
