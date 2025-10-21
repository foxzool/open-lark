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

/// 移动知识空间节点请求
#[derive(Debug, Serialize, Default)]
pub struct MoveSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 节点token
    #[serde(skip)]
    node_token: String,
    /// 目标父节点token，移动到根目录时可以为空
    #[serde(skip_serializing_if = "Option::is_none")]
    target_parent_token: Option<String>,
    /// 目标位置，移动到目标父节点的指定位置，不填时追加到末尾
    #[serde(skip_serializing_if = "Option::is_none")]
    target_prev_token: Option<String>,
}

impl MoveSpaceNodeRequest {
    pub fn builder() -> MoveSpaceNodeRequestBuilder {
        MoveSpaceNodeRequestBuilder::default()
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
pub struct MoveSpaceNodeRequestBuilder {
    request: MoveSpaceNodeRequest,
}

impl MoveSpaceNodeRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 要移动的节点token
    pub fn node_token(mut self, node_token: impl ToString) -> Self {
        self.request.node_token = node_token.to_string();
        self
    }

    /// 目标父节点token
    pub fn target_parent_token(mut self, target_parent_token: impl ToString) -> Self {
        self.request.target_parent_token = Some(target_parent_token.to_string());
        self
    }

    /// 移动到根目录
    pub fn move_to_root(mut self) -> Self {
        self.request.target_parent_token = None;
        self
    }

    /// 目标位置，移动到指定节点之后
    pub fn target_prev_token(mut self, target_prev_token: impl ToString) -> Self {
        self.request.target_prev_token = Some(target_prev_token.to_string());
        self
    }

    /// 追加到末尾
    pub fn append_to_end(mut self) -> Self {
        self.request.target_prev_token = None;
        self
    }

    pub fn build(mut self) -> MoveSpaceNodeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 移动后的节点信息
#[derive(Debug, Deserialize)]
pub struct MovedNode {
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

/// 移动知识空间节点响应
#[derive(Debug, Deserialize)]
pub struct MoveSpaceNodeResponse {
    /// 移动后的节点信息
    pub node: MovedNode,
}

impl ApiResponseTrait for MoveSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动知识空间节点
pub async fn move_space_node(
    request: MoveSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<MoveSpaceNodeResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = {
        let mut path =
            EndpointBuilder::replace_param(WIKI_V2_SPACE_NODE_MOVE, "space_id", &request.space_id);
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
    fn test_move_space_node_request_builder() {
        let request = MoveSpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .node_token("wikcnxxxxxx")
            .target_parent_token("wikcnyyyyyyy")
            .target_prev_token("wikcnzzzzzzz")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(
            request.target_parent_token,
            Some("wikcnyyyyyyy".to_string())
        );
        assert_eq!(request.target_prev_token, Some("wikcnzzzzzzz".to_string()));
    }

    #[test]
    fn test_move_to_root() {
        let request = MoveSpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .node_token("wikcnxxxxxx")
            .move_to_root()
            .append_to_end()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(request.target_parent_token, None);
        assert_eq!(request.target_prev_token, None);
    }
}
