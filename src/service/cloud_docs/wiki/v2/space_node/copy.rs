use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{cloud_docs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 复制知识空间节点请求
#[derive(Debug, Serialize, Default)]
pub struct CopySpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 节点token
    #[serde(skip)]
    node_token: String,
    /// 目标父节点token，复制到根目录时可以为空
    #[serde(skip_serializing_if = "Option::is_none")]
    target_parent_token: Option<String>,
    /// 目标知识空间id，不填时复制到当前知识空间
    #[serde(skip_serializing_if = "Option::is_none")]
    target_space_id: Option<String>,
    /// 复制后的标题，不填时使用原标题
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl CopySpaceNodeRequest {
    pub fn builder() -> CopySpaceNodeRequestBuilder {
        CopySpaceNodeRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString, node_token: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            node_token: node_token.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CopySpaceNodeRequestBuilder {
    request: CopySpaceNodeRequest,
}

impl CopySpaceNodeRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 要复制的节点token
    pub fn node_token(mut self, node_token: impl ToString) -> Self {
        self.request.node_token = node_token.to_string();
        self
    }

    /// 目标父节点token
    pub fn target_parent_token(mut self, target_parent_token: impl ToString) -> Self {
        self.request.target_parent_token = Some(target_parent_token.to_string());
        self
    }

    /// 复制到根目录
    pub fn move_to_root(mut self) -> Self {
        self.request.target_parent_token = None;
        self
    }

    /// 目标知识空间id
    pub fn target_space_id(mut self, target_space_id: impl ToString) -> Self {
        self.request.target_space_id = Some(target_space_id.to_string());
        self
    }

    /// 复制到当前空间
    pub fn move_to_current_space(mut self) -> Self {
        self.request.target_space_id = None;
        self
    }

    /// 复制后的标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    /// 使用原标题
    pub fn keep_original_title(mut self) -> Self {
        self.request.title = None;
        self
    }

    pub fn build(mut self) -> CopySpaceNodeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CopySpaceNodeRequestBuilder,
    crate::service::cloud_docs::wiki::v2::space_node::SpaceNodeService,
    CopySpaceNodeRequest,
    CopySpaceNodeResponse,
    copy
);

/// 复制后的节点信息
#[derive(Debug, Deserialize)]
pub struct CopiedNode {
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

/// 复制知识空间节点响应
#[derive(Debug, Deserialize)]
pub struct CopySpaceNodeResponse {
    /// 复制后的节点信息
    pub node: CopiedNode,
}

impl ApiResponseTrait for CopySpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制知识空间节点
pub async fn copy_space_node(
    request: CopySpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CopySpaceNodeResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = {
        let mut path =
            EndpointBuilder::replace_param(WIKI_V2_SPACE_NODE_COPY, "space_id", &request.space_id);
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
    fn test_copy_space_node_request_builder() {
        let request = CopySpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .node_token("wikcnxxxxxx")
            .target_parent_token("wikcnyyyyyyy")
            .target_space_id("spcyyyyyy")
            .title("复制的文档")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(
            request.target_parent_token,
            Some("wikcnyyyyyyy".to_string())
        );
        assert_eq!(request.target_space_id, Some("spcyyyyyy".to_string()));
        assert_eq!(request.title, Some("复制的文档".to_string()));
    }

    #[test]
    fn test_copy_to_root_current_space() {
        let request = CopySpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .node_token("wikcnxxxxxx")
            .move_to_root()
            .move_to_current_space()
            .keep_original_title()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(request.target_parent_token, None);
        assert_eq!(request.target_space_id, None);
        assert_eq!(request.title, None);
    }
}
